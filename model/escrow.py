"""Executable business specification. Does not emulate Soroban auth, TTL, or RPC."""
from dataclasses import dataclass
from copy import deepcopy
from functools import wraps

MAX_I128 = 2**127 - 1
ACTIVE = {'Funded', 'Submitted', 'Disputed'}

class EscrowError(ValueError):
    pass

def atomic(fn):
    @wraps(fn)
    def call(self, *args, **kwargs):
        before = deepcopy(self.__dict__)
        try:
            result = fn(self, *args, **kwargs)
            self.check()
            return result
        except Exception:
            self.__dict__.clear()
            self.__dict__.update(before)
            raise
    return call

@dataclass
class Milestone:
    amount: int
    due_at: int
    status: str = 'Pending'
    evidence_uri: str = ''

@dataclass
class Engagement:
    client: str
    freelancer: str
    arbitrator: str
    terms_uri: str
    milestones: list

class Escrow:
    def __init__(self, balances=None):
        self.balances = dict(balances or {})
        self.engagements = {}
        self.next_id = 0
        self.now = 0
        self.locked = 0
        self.vault = 0
        self.supply = sum(self.balances.values())

    @staticmethod
    def require(condition, message):
        if not condition:
            raise EscrowError(message)

    @staticmethod
    def uri(value):
        Escrow.require(isinstance(value, str) and 0 < len(value.encode()) <= 512, 'InvalidUri')

    def check(self):
        expected = sum(m.amount for g in self.engagements.values()
                       for m in g.milestones if m.status in ACTIVE)
        assert self.locked == expected
        assert self.vault >= self.locked >= 0
        assert all(v >= 0 for v in self.balances.values())
        assert sum(self.balances.values()) + self.vault == self.supply

    def get(self, eid, mid):
        self.require(eid in self.engagements, 'NotFound')
        g = self.engagements[eid]
        self.require(type(mid) is int and 0 <= mid < len(g.milestones), 'NotFound')
        return g, g.milestones[mid]

    def state(self, milestone, expected):
        self.require(milestone.status == expected, 'InvalidState')

    @atomic
    def create(self, client, freelancer, arbitrator, terms_uri, terms, signers):
        self.require({client, freelancer, arbitrator} <= set(signers), 'Auth')
        self.require(len({client, freelancer, arbitrator}) == 3 and
                     all(isinstance(x, str) and x for x in (client, freelancer, arbitrator))
                     and 0 < len(terms) <= 20, 'InvalidTerms')
        self.uri(terms_uri)
        self.require(all(type(a) is int and 0 < a <= MAX_I128 and
                         type(d) is int and self.now < d < 2**64 for a, d in terms), 'InvalidTerms')
        self.require(sum(a for a, _ in terms) <= MAX_I128 and self.next_id < 2**64 - 1, 'Overflow')
        eid = self.next_id
        self.next_id += 1
        self.engagements[eid] = Engagement(client, freelancer, arbitrator, terms_uri,
                                          [Milestone(a, d) for a, d in terms])
        return eid

    @atomic
    def fund(self, eid, mid, actor):
        g, m = self.get(eid, mid)
        self.require(actor == g.client, 'Auth')
        self.state(m, 'Pending')
        self.require(self.now <= m.due_at, 'Deadline')
        self.require(self.locked + m.amount <= MAX_I128, 'Overflow')
        self.require(self.balances.get(actor, 0) >= m.amount, 'InsufficientBalance')
        self.balances[actor] -= m.amount
        self.vault += m.amount
        self.locked += m.amount
        m.status = 'Funded'

    @atomic
    def submit(self, eid, mid, uri, actor):
        g, m = self.get(eid, mid)
        self.require(actor == g.freelancer, 'Auth')
        self.state(m, 'Funded')
        self.require(self.now <= m.due_at, 'Deadline')
        self.uri(uri)
        m.evidence_uri = uri
        m.status = 'Submitted'

    def release(self, g, m, refund, payout, status):
        self.require(type(refund) is int and type(payout) is int and refund >= 0
                     and payout >= 0 and refund + payout == m.amount, 'InvalidSplit')
        self.locked -= m.amount
        self.vault -= m.amount
        self.balances[g.client] = self.balances.get(g.client, 0) + refund
        self.balances[g.freelancer] = self.balances.get(g.freelancer, 0) + payout
        m.status = status

    @atomic
    def approve(self, eid, mid, actor):
        g, m = self.get(eid, mid)
        self.require(actor == g.client, 'Auth')
        self.state(m, 'Submitted')
        self.release(g, m, 0, m.amount, 'Completed')

    @atomic
    def dispute(self, eid, mid, actor):
        g, m = self.get(eid, mid)
        self.require(actor in (g.client, g.freelancer), 'WrongParty')
        self.state(m, 'Submitted')
        m.status = 'Disputed'

    @atomic
    def resolve(self, eid, mid, refund, payout, actor):
        g, m = self.get(eid, mid)
        self.require(actor == g.arbitrator, 'Auth')
        self.state(m, 'Disputed')
        self.release(g, m, refund, payout, 'Resolved')

    @atomic
    def refund(self, eid, mid, actor):
        g, m = self.get(eid, mid)
        self.require(actor == g.client, 'Auth')
        self.state(m, 'Funded')
        self.require(self.now > m.due_at, 'Deadline')
        self.release(g, m, m.amount, 0, 'Refunded')
