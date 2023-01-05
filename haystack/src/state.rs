use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Recipients {
    pub recipient: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSIT: Map<u64, Recipients> = Map::new("deposit_amount_per_address");
pub const COUNTER: Item<u64> = Item::new("counter");
