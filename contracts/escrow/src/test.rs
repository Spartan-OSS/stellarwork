use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events as _, Ledger},
    vec, IntoVal, Val, Vec,
};

fn fixture() -> (Env, Address, Address, Address, Address, Address) {
    let env = Env::default();
    env.ledger().with_mut(|l| {
        l.timestamp = 1;
    });
    let client = Address::generate(&env);
    let worker = Address::generate(&env);
    let judge = Address::generate(&env);
    let admin = Address::generate(&env);
    let asset = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    let id = env.register(Escrow, (asset.clone(),));
    // Only happy-path fixtures mock auth. Missing-signature coverage is a release gate.
    env.mock_all_auths();
    token::StellarAssetClient::new(&env, &asset).mint(&client, &1000);
    (env, id, asset, client, worker, judge)
}

fn create(env: &Env, id: &Address, c: &Address, w: &Address, j: &Address) -> u64 {
    EscrowClient::new(env, id).create_engagement(
        c,
        w,
        j,
        &String::from_str(env, "ipfs://terms"),
        &vec![
            env,
            Terms {
                amount: 100,
                due_at: 10,
            },
            Terms {
                amount: 200,
                due_at: 20,
            },
        ],
    )
}

fn assert_last_event<T, D>(env: &Env, contract: &Address, topics: T, data: D)
where
    T: IntoVal<Env, Vec<Val>>,
    D: IntoVal<Env, Val>,
{
    let all = env.events().all();
    let actual = vec![env, all.get(all.len() - 1).unwrap()];
    let expected = vec![
        env,
        (contract.clone(), topics.into_val(env), data.into_val(env)),
    ];
    assert_eq!(actual, expected);
}

#[test]
fn approval_conserves_tokens_and_preserves_other_milestone() {
    let (env, id, asset, c, w, j) = fixture();
    let api = EscrowClient::new(&env, &id);
    let eid = create(&env, &id, &c, &w, &j);
    assert_last_event(&env, &id, (symbol_short!("created"), eid), 2_u32);
    api.fund_milestone(&eid, &0);
    assert_last_event(&env, &id, (symbol_short!("funded"), eid, 0_u32), 100_i128);
    api.fund_milestone(&eid, &1);
    api.submit_work(&eid, &0, &String::from_str(&env, "ipfs://proof"));
    assert_last_event(
        &env,
        &id,
        (symbol_short!("submitted"), eid, 0_u32),
        String::from_str(&env, "ipfs://proof"),
    );
    api.approve_milestone(&eid, &0);
    assert_last_event(
        &env,
        &id,
        (symbol_short!("settled"), eid, 0_u32),
        (0_i128, 100_i128, Status::Completed),
    );
    let t = token::Client::new(&env, &asset);
    assert_eq!(t.balance(&c), 700);
    assert_eq!(t.balance(&w), 100);
    assert_eq!(t.balance(&id), 200);
    assert_eq!(api.get_locked(), 200);
    assert_eq!(api.get_milestone(&eid, &1).status, Status::Funded);
    assert!(api.try_approve_milestone(&eid, &0).is_err());
    assert!(api.try_fund_milestone(&eid, &0).is_err());
}

#[test]
fn split_resolution_and_invalid_split() {
    let (env, id, asset, c, w, j) = fixture();
    let api = EscrowClient::new(&env, &id);
    let eid = create(&env, &id, &c, &w, &j);
    api.fund_milestone(&eid, &0);
    api.submit_work(&eid, &0, &String::from_str(&env, "ipfs://proof"));
    api.raise_dispute(&eid, &0, &w);
    assert_last_event(
        &env,
        &id,
        (symbol_short!("disputed"), eid, 0_u32),
        w.clone(),
    );
    assert!(api.try_resolve_dispute(&eid, &0, &70, &40).is_err());
    assert_eq!(api.get_locked(), 100);
    assert_eq!(api.get_milestone(&eid, &0).status, Status::Disputed);
    api.resolve_dispute(&eid, &0, &70, &30);
    assert_last_event(
        &env,
        &id,
        (symbol_short!("settled"), eid, 0_u32),
        (70_i128, 30_i128, Status::Resolved),
    );
    let t = token::Client::new(&env, &asset);
    assert_eq!(t.balance(&c), 970);
    assert_eq!(t.balance(&w), 30);
    assert_eq!(t.balance(&id), 0);
    assert_eq!(api.get_locked(), 0);
    assert!(api.try_resolve_dispute(&eid, &0, &70, &30).is_err());
}

#[test]
fn refund_only_after_deadline() {
    let (env, id, asset, c, w, j) = fixture();
    let api = EscrowClient::new(&env, &id);
    let eid = create(&env, &id, &c, &w, &j);
    api.fund_milestone(&eid, &0);
    env.ledger().with_mut(|l| {
        l.timestamp = 10;
    });
    assert!(api.try_refund_expired(&eid, &0).is_err());
    env.ledger().with_mut(|l| {
        l.timestamp = 11;
    });
    assert!(api
        .try_submit_work(&eid, &0, &String::from_str(&env, "proof"))
        .is_err());
    api.refund_expired(&eid, &0);
    assert_eq!(token::Client::new(&env, &asset).balance(&c), 1000);
    assert_eq!(api.get_locked(), 0);
    assert_eq!(api.get_milestone(&eid, &0).status, Status::Refunded);
}

#[test]
fn rejects_duplicate_roles_and_outsider_disputes() {
    let (env, id, _, c, w, j) = fixture();
    let api = EscrowClient::new(&env, &id);
    assert!(api
        .try_create_engagement(
            &c,
            &c,
            &j,
            &String::from_str(&env, "terms"),
            &vec![
                &env,
                Terms {
                    amount: 100,
                    due_at: 10
                }
            ]
        )
        .is_err());
    let eid = create(&env, &id, &c, &w, &j);
    api.fund_milestone(&eid, &0);
    api.submit_work(&eid, &0, &String::from_str(&env, "proof"));
    assert!(api.try_raise_dispute(&eid, &0, &j).is_err());
}
