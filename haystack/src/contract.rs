use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{AllRecipientsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Config, CONFIG, COUNTER, DEPOSIT};
use cosmwasm_std::{
    coin, entry_point, from_binary, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Bound;

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
    env: Env,
    info: MessageInfo,
    output_address: String,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(vec![Coin::new(10_000_000, JUNO)]))?;
    let validated_output_address = deps.api.addr_validate(&output_address)?;
    let output_string = validated_output_address.to_string();
    let old_count = COUNTER.load(deps.storage)?;
    let new_count = old_count + 1;
    DEPOSIT.save(deps.storage, new_count, &output_string)?;
    //query profile name
    let msg = QueryMsg::AllRecipients {
        limit: None,
        start_after: None,
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllRecipientsResponse = from_binary(&bin).unwrap();
    let mut messages: Vec<BankMsg> = vec![];
    for depositer in res.recipients {
        let bank_msg = BankMsg::Send {
            to_address: depositer,
            amount: vec![coin(9_000_000, JUNO)],
        };
        messages.push(bank_msg);
    }
    let author_take = BankMsg::Send {
        to_address: ADMIN.to_string(),
        amount: vec![Coin::new(10_000_000, JUNO)],
    };
    messages.push(author_take);
    Ok(Response::new().add_messages(messages))
}
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllRecipients { limit, start_after } => {
            query_all_recipients(deps, env, limit, start_after)
        }
    }
}

//pagination fields
const MAX_LIMIT: u32 = 10;
const DEFAULT_LIMIT: u32 = 10;

fn query_all_recipients(
    deps: Deps,
    _env: Env,
    limit: Option<u32>,
    start_after: Option<u64>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let recipients = DEPOSIT
        .range(deps.storage, None, start, Order::Ascending)
        .take(limit)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllRecipientsResponse { recipients })
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
