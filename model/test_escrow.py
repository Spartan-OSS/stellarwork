import unittest
import random
from copy import deepcopy
from escrow import Escrow, EscrowError, MAX_I128

class EscrowTests(unittest.TestCase):
    def setUp(self):
        self.e = Escrow({'client': 1000})
        self.id = self.e.create('client', 'worker', 'judge', 'ipfs://terms',
                                [(100, 10), (200, 20)], ['client', 'worker', 'judge'])

    def rejected(self, fn, *args):
        before = deepcopy(self.e.__dict__)
        with self.assertRaises(EscrowError):
            fn(*args)
        self.assertEqual(before, self.e.__dict__)

    def submitted(self):
        self.e.fund(self.id, 0, 'client')
        self.e.submit(self.id, 0, 'ipfs://proof', 'worker')

    def test_happy_path_and_replay(self):
        self.submitted()
        self.e.approve(self.id, 0, 'client')
        self.assertEqual(self.e.balances['worker'], 100)
        self.assertEqual(self.e.locked, 0)
        self.rejected(self.e.approve, self.id, 0, 'client')
        self.rejected(self.e.fund, self.id, 0, 'client')

    def test_dispute_split_and_replay(self):
        self.submitted()
        self.e.dispute(self.id, 0, 'worker')
        self.e.resolve(self.id, 0, 70, 30, 'judge')
        self.assertEqual(self.e.balances, {'client': 970, 'worker': 30})
        self.rejected(self.e.resolve, self.id, 0, 70, 30, 'judge')
        self.rejected(self.e.approve, self.id, 0, 'client')

    def test_split_endpoints_and_rounding(self):
        for refund in (0, 1, 33, 99, 100):
            e = Escrow({'c': 100})
            i = e.create('c', 'w', 'j', 'terms', [(100, 10)], ['c', 'w', 'j'])
            e.fund(i, 0, 'c'); e.submit(i, 0, 'proof', 'w'); e.dispute(i, 0, 'c')
            e.resolve(i, 0, refund, 100-refund, 'j')
            self.assertEqual(e.balances['c'], refund)
            self.assertEqual(e.balances['w'], 100-refund)

    def test_invalid_splits_are_atomic(self):
        self.submitted(); self.e.dispute(self.id, 0, 'client')
        for a, b in [(-1, 101), (101, -1), (40, 40), (60, 50), (MAX_I128, 1)]:
            self.rejected(self.e.resolve, self.id, 0, a, b, 'judge')

    def test_all_roles(self):
        self.rejected(self.e.fund, self.id, 0, 'worker')
        self.e.fund(self.id, 0, 'client')
        self.rejected(self.e.submit, self.id, 0, 'proof', 'client')
        self.e.submit(self.id, 0, 'proof', 'worker')
        self.rejected(self.e.approve, self.id, 0, 'worker')
        self.rejected(self.e.dispute, self.id, 0, 'outsider')
        self.e.dispute(self.id, 0, 'client')
        self.rejected(self.e.resolve, self.id, 0, 50, 50, 'client')

    def test_deadline_boundary(self):
        self.e.now = 10; self.e.fund(self.id, 0, 'client')
        self.rejected(self.e.refund, self.id, 0, 'client')
        self.e.submit(self.id, 0, 'proof', 'worker')
        self.e.now = 11
        self.rejected(self.e.refund, self.id, 0, 'client')
        self.e.approve(self.id, 0, 'client')

    def test_expired_refund(self):
        self.e.fund(self.id, 0, 'client'); self.e.now = 11
        self.rejected(self.e.submit, self.id, 0, 'proof', 'worker')
        self.rejected(self.e.refund, self.id, 0, 'worker')
        self.e.refund(self.id, 0, 'client')
        self.assertEqual(self.e.balances['client'], 1000)
        self.rejected(self.e.refund, self.id, 0, 'client')

    def test_invalid_terms_and_consent(self):
        for terms in ([], [(0, 10)], [(-1, 10)], [(1, 0)], [(1, 10)]*21,
                      [(MAX_I128, 10), (1, 10)]):
            self.rejected(self.e.create, 'client', 'worker', 'judge', 'terms', terms,
                          ['client', 'worker', 'judge'])
        self.rejected(self.e.create, 'client', 'worker', 'judge', 'terms', [(1, 10)], ['client'])
        self.rejected(self.e.create, 'client', 'client', 'judge', 'terms', [(1, 10)], ['client', 'judge'])

    def test_uri_bounds(self):
        self.e.fund(self.id, 0, 'client')
        for uri in ('', 'x'*513, 'é'*257):
            self.rejected(self.e.submit, self.id, 0, uri, 'worker')
        self.e.submit(self.id, 0, 'x'*512, 'worker')

    def test_invalid_ids(self):
        for eid, mid in [(99, 0), (self.id, -1), (self.id, 2)]:
            self.rejected(self.e.fund, eid, mid, 'client')

    def test_insufficient_funds_and_duplicate_funding(self):
        self.e.balances['client'] = 50; self.e.supply = 50
        self.rejected(self.e.fund, self.id, 0, 'client')
        self.e.balances['client'] = 1000; self.e.supply = 1000
        self.e.fund(self.id, 0, 'client')
        self.rejected(self.e.fund, self.id, 0, 'client')

    def test_isolation_between_engagements(self):
        other = self.e.create('client', 'worker2', 'judge', 'terms', [(300, 30)],
                              ['client', 'worker2', 'judge'])
        self.submitted(); self.e.fund(other, 0, 'client')
        self.e.approve(self.id, 0, 'client')
        self.assertEqual(self.e.locked, 300)
        self.assertEqual(self.e.engagements[other].milestones[0].status, 'Funded')
        self.assertNotIn('worker2', self.e.balances)

    def test_randomized_conservation(self):
        rng = random.Random(20261003)
        for _ in range(250):
            e = Escrow({'c': 1000})
            i = e.create('c', 'w', 'j', 'terms', [(100, 10), (200, 10)], ['c','w','j'])
            for _ in range(40):
                mid = rng.randrange(2)
                operation = rng.choice([
                    lambda: e.fund(i, mid, rng.choice(['c','w'])),
                    lambda: e.submit(i, mid, 'proof', rng.choice(['c','w'])),
                    lambda: e.approve(i, mid, rng.choice(['c','w'])),
                    lambda: e.dispute(i, mid, rng.choice(['c','w','j'])),
                    lambda: e.resolve(i, mid, rng.randrange(201), 0, 'j'),
                    lambda: e.refund(i, mid, 'c'),
                ])
                before = deepcopy(e.__dict__)
                try:
                    operation()
                except EscrowError:
                    self.assertEqual(before, e.__dict__)
                e.check()
                e.now += rng.randrange(2)

if __name__ == '__main__':
    unittest.main()
