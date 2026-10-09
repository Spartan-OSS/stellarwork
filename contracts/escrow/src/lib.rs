#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env,
    String, Vec,
};

const TTL_THRESHOLD: u32 = 17_280;
const TTL_EXTEND: u32 = 518_400;
const MAX_MILESTONES: u32 = 20;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Pending,
    Funded,
    Submitted,
    Completed,
    Disputed,
    Resolved,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Terms {
    pub amount: i128,
    pub due_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub amount: i128,
    pub due_at: u64,
    pub status: Status,
    pub evidence_uri: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Engagement {
    pub client: Address,
    pub freelancer: Address,
    pub arbitrator: Address,
    pub terms_uri: String,
    pub count: u32,
}

#[contracttype]
#[derive(Clone)]
enum Key {
    Token,
    NextId,
    Locked,
    Engagement(u64),
    Milestone(u64, u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    InvalidTerms = 2,
    InvalidState = 3,
    WrongParty = 4,
    Deadline = 5,
    InvalidUri = 6,
    InvalidSplit = 7,
    Overflow = 8,
}

#[contractevent(topics = ["created"])]
pub struct CreatedEvent {
    #[topic]
    pub id: u64,
    pub count: u32,
}

#[contractevent(topics = ["funded"])]
pub struct FundedEvent {
    #[topic]
    pub id: u64,
    #[topic]
    pub mid: u32,
    pub amount: i128,
}

#[contractevent(topics = ["submitted"])]
pub struct SubmittedEvent {
    #[topic]
    pub id: u64,
    #[topic]
    pub mid: u32,
    pub evidence_uri: String,
}

#[contractevent(topics = ["disputed"])]
pub struct DisputedEvent {
    #[topic]
    pub id: u64,
    #[topic]
    pub mid: u32,
    pub actor: Address,
}

#[contractevent(topics = ["settled"])]
pub struct SettledEvent {
    #[topic]
    pub id: u64,
    #[topic]
    pub mid: u32,
    pub refund: i128,
    pub payout: i128,
    pub status: Status,
}

#[contract]
pub struct Escrow;

fn touch(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND);
}
fn token_address(env: &Env) -> Address {
    env.storage().instance().get(&Key::Token).unwrap()
}
fn load_engagement(env: &Env, id: u64) -> Result<Engagement, Error> {
    let key = Key::Engagement(id);
    let value = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::NotFound)?;
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND);
    touch(env);
    Ok(value)
}
fn load_milestone(env: &Env, id: u64, mid: u32) -> Result<Milestone, Error> {
    let key = Key::Milestone(id, mid);
    let value = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::NotFound)?;
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND);
    Ok(value)
}
fn save_milestone(env: &Env, id: u64, mid: u32, m: &Milestone) {
    let key = Key::Milestone(id, mid);
    env.storage().persistent().set(&key, m);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND);
}
fn valid_uri(uri: &String) -> Result<(), Error> {
    if uri.len() == 0 || uri.len() > 512 {
        return Err(Error::InvalidUri);
    }
    Ok(())
}
fn locked(env: &Env) -> i128 {
    env.storage().instance().get(&Key::Locked).unwrap_or(0)
}
fn release(
    env: &Env,
    id: u64,
    mid: u32,
    m: &mut Milestone,
    g: &Engagement,
    refund: i128,
    payout: i128,
    status: Status,
) -> Result<(), Error> {
    if refund < 0 || payout < 0 || refund.checked_add(payout) != Some(m.amount) {
        return Err(Error::InvalidSplit);
    }
    let remaining = locked(env).checked_sub(m.amount).ok_or(Error::Overflow)?;
    if remaining < 0 {
        return Err(Error::InvalidSplit);
    }
    m.status = status;
    save_milestone(env, id, mid, m);
    env.storage().instance().set(&Key::Locked, &remaining);
    let t = token::Client::new(env, &token_address(env));
    // Soroban invocation rollback must revert state AND both transfers on failure.
    if refund > 0 {
        t.transfer(&env.current_contract_address(), &g.client, &refund);
    }
    if payout > 0 {
        t.transfer(&env.current_contract_address(), &g.freelancer, &payout);
    }
    SettledEvent {
        id,
        mid,
        refund,
        payout,
        status: m.status.clone(),
    }
    .publish(env);
    Ok(())
}

#[contractimpl]
impl Escrow {
    // One reviewed Stellar Asset Contract per deployment; no upgrade/admin withdrawal.
    pub fn __constructor(env: Env, token: Address) {
        env.storage().instance().set(&Key::Token, &token);
        env.storage().instance().set(&Key::NextId, &0_u64);
        env.storage().instance().set(&Key::Locked, &0_i128);
        touch(&env);
    }

    pub fn create_engagement(
        env: Env,
        client: Address,
        freelancer: Address,
        arbitrator: Address,
        terms_uri: String,
        terms: Vec<Terms>,
    ) -> Result<u64, Error> {
        client.require_auth();
        freelancer.require_auth();
        arbitrator.require_auth();
        if client == freelancer
            || client == arbitrator
            || freelancer == arbitrator
            || terms.len() == 0
            || terms.len() > MAX_MILESTONES
        {
            return Err(Error::InvalidTerms);
        }
        valid_uri(&terms_uri)?;
        let mut total = 0_i128;
        for term in terms.iter() {
            if term.amount <= 0 || term.due_at <= env.ledger().timestamp() {
                return Err(Error::InvalidTerms);
            }
            total = total.checked_add(term.amount).ok_or(Error::Overflow)?;
        }
        let id: u64 = env.storage().instance().get(&Key::NextId).unwrap();
        let next = id.checked_add(1).ok_or(Error::Overflow)?;
        let g = Engagement {
            client,
            freelancer,
            arbitrator,
            terms_uri,
            count: terms.len(),
        };
        let key = Key::Engagement(id);
        env.storage().persistent().set(&key, &g);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND);
        for (mid, term) in terms.iter().enumerate() {
            let m = Milestone {
                amount: term.amount,
                due_at: term.due_at,
                status: Status::Pending,
                evidence_uri: String::from_str(&env, ""),
            };
            save_milestone(&env, id, mid as u32, &m);
        }
        env.storage().instance().set(&Key::NextId, &next);
        touch(&env);
        CreatedEvent { id, count: g.count }.publish(&env);
        Ok(id)
    }

    pub fn fund_milestone(env: Env, id: u64, mid: u32) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        g.client.require_auth();
        let mut m = load_milestone(&env, id, mid)?;
        if m.status != Status::Pending {
            return Err(Error::InvalidState);
        }
        if env.ledger().timestamp() > m.due_at {
            return Err(Error::Deadline);
        }
        let next = locked(&env).checked_add(m.amount).ok_or(Error::Overflow)?;
        m.status = Status::Funded;
        save_milestone(&env, id, mid, &m);
        env.storage().instance().set(&Key::Locked, &next);
        token::Client::new(&env, &token_address(&env)).transfer(
            &g.client,
            &env.current_contract_address(),
            &m.amount,
        );
        FundedEvent {
            id,
            mid,
            amount: m.amount,
        }
        .publish(&env);
        Ok(())
    }

    pub fn submit_work(env: Env, id: u64, mid: u32, uri: String) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        g.freelancer.require_auth();
        let mut m = load_milestone(&env, id, mid)?;
        if m.status != Status::Funded {
            return Err(Error::InvalidState);
        }
        if env.ledger().timestamp() > m.due_at {
            return Err(Error::Deadline);
        }
        valid_uri(&uri)?;
        m.status = Status::Submitted;
        m.evidence_uri = uri;
        save_milestone(&env, id, mid, &m);
        SubmittedEvent {
            id,
            mid,
            evidence_uri: m.evidence_uri.clone(),
        }
        .publish(&env);
        Ok(())
    }

    pub fn approve_milestone(env: Env, id: u64, mid: u32) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        g.client.require_auth();
        let mut m = load_milestone(&env, id, mid)?;
        if m.status != Status::Submitted {
            return Err(Error::InvalidState);
        }
        let amount = m.amount;
        release(&env, id, mid, &mut m, &g, 0, amount, Status::Completed)
    }

    pub fn raise_dispute(env: Env, id: u64, mid: u32, actor: Address) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        if actor != g.client && actor != g.freelancer {
            return Err(Error::WrongParty);
        }
        actor.require_auth();
        let mut m = load_milestone(&env, id, mid)?;
        if m.status != Status::Submitted {
            return Err(Error::InvalidState);
        }
        m.status = Status::Disputed;
        save_milestone(&env, id, mid, &m);
        DisputedEvent { id, mid, actor }.publish(&env);
        Ok(())
    }

    pub fn resolve_dispute(
        env: Env,
        id: u64,
        mid: u32,
        refund: i128,
        payout: i128,
    ) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        g.arbitrator.require_auth();
        let mut m = load_milestone(&env, id, mid)?;
        if m.status != Status::Disputed {
            return Err(Error::InvalidState);
        }
        release(&env, id, mid, &mut m, &g, refund, payout, Status::Resolved)
    }

    pub fn refund_expired(env: Env, id: u64, mid: u32) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        g.client.require_auth();
        let mut m = load_milestone(&env, id, mid)?;
        if m.status != Status::Funded {
            return Err(Error::InvalidState);
        }
        if env.ledger().timestamp() <= m.due_at {
            return Err(Error::Deadline);
        }
        let amount = m.amount;
        release(&env, id, mid, &mut m, &g, amount, 0, Status::Refunded)
    }

    pub fn get_engagement(env: Env, id: u64) -> Result<Engagement, Error> {
        load_engagement(&env, id)
    }
    pub fn get_milestone(env: Env, id: u64, mid: u32) -> Result<Milestone, Error> {
        load_engagement(&env, id)?;
        load_milestone(&env, id, mid)
    }
    pub fn get_locked(env: Env) -> i128 {
        touch(&env);
        locked(&env)
    }
    pub fn get_token(env: Env) -> Address {
        touch(&env);
        token_address(&env)
    }
    // Permissionless rent maintenance; caller pays. Archived entries need RPC restoration.
    pub fn keep_alive(env: Env, id: u64) -> Result<(), Error> {
        let g = load_engagement(&env, id)?;
        for mid in 0..g.count {
            load_milestone(&env, id, mid)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test;
