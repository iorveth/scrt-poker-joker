use crate::game::GameDetails;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::game::GameStatus;
use crate::game::NUM_OF_DICES;

use crate::contract::{GameId, Secret};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateNewGameRoom {
        nft_id: String,
        base_bet: Coin,
        secret: Secret,
    },
    JoinGame {
        nft_id: String,
        game_id: GameId,
        secret: Secret,
    },
    Roll {
        game_id: GameId,
    },
    ReRoll {
        game_id: GameId,
        dices: [bool; NUM_OF_DICES],
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // retrieve all games by status provided
    GamesByStatus { status: GameStatus },
    // get game under specified id
    Game { game_id: GameId },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct GameResonse(GameDetails);

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct ActiveGamesResponse(Vec<GameDetails>);

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct PendingGamesResponse(Vec<GameDetails>);
