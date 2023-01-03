#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError, entry_point};
use cw2::{set_contract_version, get_contract_version};

use crate::error::ContractError;
use crate::state::{CONFIG, Config, DEPOSITAMOUNT};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

//Admin wallet
const ADMIN: &str = "juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945";

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
    CONFIG.save(deps.storage, &config)?;
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
        ExecuteMsg::Deposit {  } => execute_deposit(deps, env, info),
    }
}
fn execute_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let check = DEPOSITAMOUNT.may_load(deps.storage, info.sender.clone())?;
    match check {
        Some(check) => {
            for coin in info.funds {
                if coin.denom == "ujuno" {
                    let total_amount = coin.amount + check;
                    DEPOSITAMOUNT.save(deps.storage, info.sender.clone(), &total_amount)?;
                }
                else {
                    continue
                }
            }
            Ok(Response::new())
        }
        None => {
            for coin in info.funds {
                if coin.denom == "ujuno" {
                    let sent_amount = coin.amount;
                    DEPOSITAMOUNT.save(deps.storage, info.sender.clone(), &sent_amount)?;
                }
                else {
                    continue
                }
            }
            Ok(Response::new())
        }
    }
}

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
