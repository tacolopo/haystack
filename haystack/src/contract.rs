use std::str::FromStr;

use cosmwasm_std::Uint128;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Coin, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError, entry_point};
use cw2::{set_contract_version, get_contract_version};

use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::state::{CONFIG, Config, DEPOSITAMOUNT, TOTALDEPOSITS, ADDRESSES, Depositor};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

//Admin wallet
const ADMIN: &str = "juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945";
//denom
const JUNO: &str = "ujunox";

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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let validated_admin = deps.api.addr_validate(ADMIN)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    let zero = Uint128::from_str("0")?;
    CONFIG.save(deps.storage, &config)?;
    TOTALDEPOSITS.save(deps.storage, &zero)?;
    ADDRESSES.save(deps.storage, &0)?;
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
        ExecuteMsg::Withdraw {  } => execute_withdraw(deps, env, info)
    }
}
fn execute_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    output_address: String,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(vec![Coin::new(1_000_000, JUNO)]))?;
    let check = DEPOSITAMOUNT.may_load(deps.storage, info.sender.clone())?;
    match check {
        Some(check) => {
            for coin in info.funds {
                if coin.denom == JUNO {
                    // check sent output string
                    let validated_output_address = deps.api.addr_validate(&output_address)?;
                    let updated_depositor_info = Depositor { recipient: validated_output_address, amount: coin.amount + check.amount };
                    DEPOSITAMOUNT.save(deps.storage, info.sender.clone(), &updated_depositor_info)?;
                    //update total deposits
                    let current_deposits = TOTALDEPOSITS.load(deps.storage)?;
                    let total_deposits = updated_depositor_info.amount + current_deposits;
                    TOTALDEPOSITS.save(deps.storage, &total_deposits)?;
                    //update addresses
                    let current_addresses = ADDRESSES.load(deps.storage)?;
                    let total_addresses = current_addresses + 1;
                    ADDRESSES.save(deps.storage, &total_addresses)?;
                }
                else {
                    continue
                }
            }
            Ok(Response::new())
        }
        None => {
            for coin in info.funds {
                if coin.denom == JUNO {
                    // check sent output string
                    let validated_output_address = deps.api.addr_validate(&output_address)?;
                    let updated_depositor_info = Depositor { recipient: validated_output_address, amount: coin.amount };
                    DEPOSITAMOUNT.save(deps.storage, info.sender.clone(), &updated_depositor_info)?;
                    //update total deposits
                    let current_deposits = TOTALDEPOSITS.load(deps.storage)?;
                    let total_deposits = updated_depositor_info.amount + current_deposits;
                    TOTALDEPOSITS.save(deps.storage, &total_deposits)?;
                    //update addresses
                    let current_addresses = ADDRESSES.load(deps.storage)?;
                    let total_addresses = current_addresses + 1;
                    ADDRESSES.save(deps.storage, &total_addresses)?;
                }
                else {
                    continue
                }
            }
            Ok(Response::new())
        }
    }
}
// fn execute_withdraw(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
// ) -> Result<Response, ContractError> {
//     assert_sent_exact_coin(&info.funds, Some(vec![Coin::new(10_000_000, JUNO)]))?;
//     let 

//     Ok(Response::new())
// }

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    //canonical way from official docs https://docs.cosmwasm.com/docs/1.0/smart-contracts/migration/#migrate-which-updates-the-version-only-if-newer
    #[allow(clippy::cmp_owned)]
    if ver.version > (*CONTRACT_VERSION).to_string() {
        return Err(StdError::generic_err("Must upgrade from a lower version").into());
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default()
        .add_attribute("action", "migration")
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("contract", CONTRACT_NAME))
}
