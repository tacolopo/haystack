use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Depositor {
    pub recipient: Addr,
    pub amount: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSITAMOUNT: Map<Addr, Depositor> = Map::new("deposit_amount_per_address");
pub const TOTALDEPOSITS: Item<Uint128> = Item::new("total_deposits");
pub const ADDRESSES: Item<u64> = Item::new("addresses");

