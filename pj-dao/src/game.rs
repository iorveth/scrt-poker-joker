use crate::error::{ContractError, ContractResult};
use cosmwasm_std::{coin, CanonicalAddr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const TOTAL_ROUNDS: usize = 2;

// Max num of dices each player rolls in the game round
pub const NUM_OF_DICES: usize = 5;

// (5 dices) x 2 rounds
pub type Rolls = [Option<[u8; NUM_OF_DICES]>; TOTAL_ROUNDS];

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
}

impl GameDetails {
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
            ..GameDetails::default()
        }
    }

    /// Join the game
    pub fn join(&mut self, joined_player_address: CanonicalAddr, joined_player_nft_id: String) {
        self.joined_player_address = joined_player_address;
        self.joined_player_nft_id = joined_player_nft_id;

        // game started
        self.status = GameStatus::Started;
    }

    /// Roll dices
    pub fn roll(&mut self, player: Player) {
        match player {
            Player::Host => {}
            Player::Joined => {}
        }

        // game started
        self.status = GameStatus::ReRoll;
    }

    /// Reroll chosen dices
    /// false - do not reroll
    /// true - reroll
    pub fn reroll(&mut self, player: Player, dices: [bool; 5]) {
        match player {
            Player::Host => {}
            Player::Joined => {}
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

impl Default for GameStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl GameStatus {
    /// Ensure GameStatus is set to Pending
    pub fn ensure_is_pending(&self) -> ContractResult<()> {
        if self.ne(&GameStatus::Pending) {
            Err(ContractError::GameNotInPendingStatus {})
        } else {
            Ok(())
        }
    }

    /// Ensure GameStatus is set to Started
    pub fn ensure_is_started(&self) -> ContractResult<()> {
        if self.ne(&GameStatus::Started) {
            Err(ContractError::GameNotInStartedStatus {})
        } else {
            Ok(())
        }
    }

    /// Ensure GameStatus is set to Reroll
    pub fn ensure_is_reroll(&self) -> ContractResult<()> {
        if self.ne(&GameStatus::ReRoll) {
            Err(ContractError::GameNotInRerollStatus {})
        } else {
            Ok(())
        }
    }
}
