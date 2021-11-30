use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GameDetails {
    pub status: GameStatus,
    pub shielded: bool,
    pub initNftId: String,
    pub joinedNftId: Option<String>,
    pub baseBet: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Pending,
    Active,
    Closed,
}
