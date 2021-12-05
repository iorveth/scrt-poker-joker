use crate::error::{ContractError, ContractResult};
use cosmwasm_std::{coin, CanonicalAddr, Coin, StdError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// total roll rounds
const TOTAL_ROUNDS: usize = 2;

// Max num of dices each player rolls in the game round
pub const NUM_OF_DICES: usize = 5;

// (5 dices) x 2 rounds
pub type Rolls = [Option<[u8; NUM_OF_DICES]>; TOTAL_ROUNDS];

// Secret bytes provided by the player
pub type Secret = [u8; 8];

// An amount locked per player for a game
pub fn locked_per_player(base_bet: &Coin) -> Coin {
    coin(
        base_bet.amount.u128() * NUM_OF_DICES as u128 * TOTAL_ROUNDS as u128,
        &base_bet.denom,
    )
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct GameDetails {
    // information about the game
    pub game: Game,
    // secret bytes provided by host player
    pub host_player_secret: Secret,
    // secret bytes provided by joined player
    pub joined_player_secret: Secret,
}

impl From<GameDetails> for Game {
    fn from(game_details: GameDetails) -> Self {
        game_details.game
    }
}

impl GameDetails {
    pub fn new(game: Game, host_player_secret: Secret) -> GameDetails {
        Self {
            game,
            host_player_secret,
            joined_player_secret: Secret::default(),
        }
    }

    /// Join the game
    pub fn join(
        &mut self,
        joined_player_address: CanonicalAddr,
        joined_player_nft_id: String,
        joined_player_secret: Secret,
    ) {
        self.game.joined_player_address = joined_player_address;
        self.game.joined_player_nft_id = joined_player_nft_id;
        self.joined_player_secret = joined_player_secret;

        // game started
        self.game.status = GameStatus::Started;
    }

    /// Roll dices
    pub fn roll(&mut self) {
        match self.game.roll_turn {
            Player::Host => {
                self.game.roll_turn = Player::Joined;
            }
            Player::Joined => {
                self.game.roll_turn = Player::Host;
            }
        }

        // Move to the reroll stage
        if self.game.host_player_rolls[0].is_some() && self.game.joined_player_rolls[0].is_some() {
            self.game.status = GameStatus::ReRoll;
        }
    }

    /// Reroll chosen dices
    /// false - do not reroll
    /// true - reroll
    pub fn reroll(&mut self, dices: [bool; 5]) {
        match self.game.roll_turn {
            Player::Host => {
                self.game.roll_turn = Player::Joined;
            }
            Player::Joined => {
                self.game.roll_turn = Player::Host;
            }
        }
    }

    /// Ensure GameStatus is set to Pending
    pub fn ensure_is_pending(&self) -> ContractResult<()> {
        if self.game.status.ne(&GameStatus::Pending) {
            Err(StdError::generic_err(
                ContractError::GameNotInPendingStatus {}.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Ensure GameStatus is set to Started
    pub fn ensure_is_started(&self) -> ContractResult<()> {
        if self.game.status.ne(&GameStatus::Started) {
            Err(StdError::generic_err(
                ContractError::GameNotInStartedStatus {}.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Ensure GameStatus is set to Reroll
    pub fn ensure_is_reroll(&self) -> ContractResult<()> {
        if self.game.status.ne(&GameStatus::ReRoll) {
            Err(StdError::generic_err(
                ContractError::GameNotInRerollStatus {}.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Ensure given account can make a roll in the game
    pub fn ensure_can_make_a_roll(&self, address: CanonicalAddr) -> ContractResult<()> {
        let can_roll = match self.game.roll_turn {
            Player::Host => self.game.host_player_address == address,
            Player::Joined => self.game.joined_player_address == address,
        };

        if !can_roll {
            Err(StdError::generic_err(
                ContractError::GivenAccountCannotMakeARoll {}.to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Whether the game is finished
    pub fn is_finished(&self) -> bool {
        // check whether both players rolled a second time
        self.game.host_player_rolls[1].is_some() && self.game.joined_player_rolls[1].is_some()
    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct Game {
    pub status: GameStatus,
    // whether the game is shielded
    pub shielded: bool,
    pub host_player_address: CanonicalAddr,
    pub joined_player_address: CanonicalAddr,
    pub host_player_nft_id: String,
    pub joined_player_nft_id: String,
    // base bet per each dice
    pub base_bet: Coin,
    // host player roll results (5 dices) x 2 rounds
    pub host_player_rolls: Rolls,
    // joined player roll results (5 dices) x 2 rounds
    pub joined_player_rolls: Rolls,
    // total game stake
    pub total_stake: Coin,
    // total game pool
    pub total_pool: Coin,
    // total points amount scored throughout the game by host player
    pub host_player_total_points: u8,
    // total points amount scored throughout the game by joined player
    pub joined_player_total_points: u8,

    // who rolls next
    pub roll_turn: Player,
}

impl Game {
    pub fn new(
        host_player_address: CanonicalAddr,
        host_player_nft_id: String,
        base_bet: Coin,
    ) -> Self {
        Self {
            status: GameStatus::Pending,
            shielded: false,
            host_player_address,
            host_player_nft_id,
            total_pool: locked_per_player(&base_bet),
            base_bet,
            ..Game::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Pending,
    Started,
    ReRoll,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Player {
    Host,
    Joined,
}

impl Default for Player {
    fn default() -> Self {
        Self::Host
    }
}

impl Default for GameStatus {
    fn default() -> Self {
        Self::Pending
    }
}
