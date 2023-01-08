use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSIT: Map<u32, String> = Map::new("deposit");
pub const COUNTER: Item<u32> = Item::new("counter");
