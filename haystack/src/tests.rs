use cosmwasm_std::attr;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use crate::msg::InstantiateMsg;
use crate::contract::instantiate;

pub const ADDR1: &str = "secret1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945";

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
