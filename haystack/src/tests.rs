use crate::contract::{execute, instantiate};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, coin};

pub const ADDR1: &str = "secret1993j5gsv2m3eqlkh7a9hvv8qdrwyr0k7pq5tua";

const SCRT: &str = "uscrt";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}

#[test]
fn test_execute_deposit() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute one deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_deposit_10() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute one deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute two deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute three deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute four deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute five deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute six deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute seven deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute eight deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute nine deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //execute ten deposit
    let info = mock_info(ADDR1, &[coin(10_000_000, SCRT)]);
    let msg = ExecuteMsg::Deposit {
        output_address: "secret1f80fazgdukg2wg6wcxhcvau396kdrxjvltp20d".to_string(),
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
