use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::NUM_OF_DICES;

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GameDetails {
    pub status: GameStatus,
    // whether the game is shielded
    pub shielded: bool,
    pub host_player_address: HumanAddr,
    pub joined_player_address: HumanAddr,
    pub host_player_nft_id: String,
    pub joined_player_nft_id: String,
    // base bet per each dice
    pub base_bet: Coin,
    // host player roll results (5 dices)
    pub host_player_current_roll: [u8; NUM_OF_DICES],
    // joined player roll results (5 dices)
    pub joined_player_current_roll: [u8; NUM_OF_DICES],
    // total game pool
    pub total_stake: Coin,
    // total points amount scored throughout the game by host player
    pub host_player_total_points: u8,
    // total points amount scored throughout the game by joined player
    pub joined_player_total_points: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Pending,
    Started,
    ReRoll,
}
