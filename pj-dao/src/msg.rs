use crate::game::GameDetails;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::NUM_OF_DICES;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateDiceNFTContract { code_id: u64 },
    JoinDao {},
    StartNewGame { nft_id: String, base_bet: Coin, secret: u64 },
    JoinGame { nft_id: String, game_id: u32, secret: u64 },
    Roll { game_id: u32 },
    ReRoll { game_id: u32, dices: [bool; NUM_OF_DICES] },
    EndGame { game_id: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ActiveGames {},
    PendingGames {},
    Game { game_id: u32 },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct GameResonse(GameDetails);

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct ActiveGamesResponse(Vec<GameDetails>);

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct PendingGamesResponse(Vec<GameDetails>);
