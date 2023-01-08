use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, CONFIG, COUNTER, DEPOSIT};
use cosmwasm_std::{coin, entry_point, BankMsg, Coin, DepsMut, Env, MessageInfo, Response};


//Admin wallet
const ADMIN: &str = "secret1993j5gsv2m3eqlkh7a9hvv8qdrwyr0k7pq5tua";
//denom
const SCRT: &str = "uscrt";
const WITHDRAW: &str = "secret1eunlgl0l8w4h70m77g5f2jkqn53qvw4k637qzw";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    let validated_admin = deps.api.addr_validate(ADMIN)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    COUNTER.save(deps.storage, &0)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { output_address } => execute_deposit(deps, env, info, output_address),
    }
}
fn execute_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    output_address: String,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(vec![Coin::new(10_000_000, SCRT)]))?;
    let validated_output_address = deps.api.addr_validate(&output_address)?;
    let output_string = validated_output_address.to_string();
    let old_count = COUNTER.load(deps.storage)?;
    let new_count = old_count + 1;
    DEPOSIT.save(deps.storage, new_count, &output_string)?;
    if new_count == 10 {
        //create vec of all recipients
        let mut res: Vec<String> = vec![];
        for count in 1..11 {
            let address = DEPOSIT.load(deps.storage, count)?;
            res.push(address);
        }
        let mut messages: Vec<BankMsg> = vec![];
        for depositer in res {
            let bank_msg = BankMsg::Send {
                to_address: depositer,
                amount: vec![coin(9_000_000, SCRT)],
            };
            messages.push(bank_msg);
        }
        let author_take = BankMsg::Send {
            to_address: WITHDRAW.to_string(),
            amount: vec![Coin::new(10_000_000, SCRT)],
        };
        messages.push(author_take);
        for num in 1..11 {
            DEPOSIT.remove(deps.storage, num);
        }
        COUNTER.save(deps.storage, &0)?;
        Ok(Response::new().add_messages(messages))
    } else {
        COUNTER.save(deps.storage, &new_count)?;
        Ok(Response::new())
    }
}
