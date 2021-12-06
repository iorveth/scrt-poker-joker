use crate::contract::GameId;
use crate::error::{ContractError, ContractResult};
use cosmwasm_std::{coin, CanonicalAddr, Coin, StdError};
use rand::Rng;
use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// total roll rounds
const TOTAL_ROUNDS: usize = 2;

// Max num of dices each player rolls in the game round
pub const NUM_OF_DICES: usize = 5;

pub const MIN_DICE_NUMBER: u8 = 1;
pub const MAX_DICE_NUMBER: u8 = 6;

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

// An amount locked per player for a game
// pub fn add_stake(&mut game_pool, &mut base_bet: &Coin) -> Coin {
//     coin(
//         base_bet.amount.u128() * NUM_OF_DICES as u128 * TOTAL_ROUNDS as u128,
//         &base_bet.denom,
//     )
// }

/// Calculate total player points
pub fn calculate_player_total_points(roll: [u8; NUM_OF_DICES]) -> u8 {
    // results table [number of equal dices], where index is a number

    // 0 - 1
    // 1 - 2
    // 2 - 3
    // 3 - 4
    // 4 - 5
    // 5 - 6

    let mut results: [u8; MAX_DICE_NUMBER as usize] = [0; MAX_DICE_NUMBER as usize];

    for i in 0..MAX_DICE_NUMBER {
        for dice in roll {
            if dice == (i + 1) {
                results[i as usize] += 1;
            }
        }
    }

    // 5 points: 5 of 1s
    if results.iter().any(|item| *item == 5) {
        5

    // 4 points: straight (1-5)
    } else if results.iter().all(|item| *item == 1) {
        4

    // 4 points: 3 of a kind + 1 pair
    } else if results.iter().any(|item| *item == 3) && results.iter().any(|item| *item == 2) {
        4

    // 3 points: 3 of a kind
    } else if results.iter().any(|item| *item == 3) {
        3

    // 2 points: 2 pairs
    } else if results.iter().filter(|item| **item == 2).count() == 2 {
        2

    // 1 point: 1 pair (it seems that we can't have less)
    } else {
        1
    }
}

/// Reroll cchosen dices
pub fn complete_reroll(rng: &mut ChaChaRng, mut roll: [u8; NUM_OF_DICES], dices: [bool; NUM_OF_DICES]) -> [u8; NUM_OF_DICES] {
    for i in 0..roll.len() {
        // reroll
        if dices[i] {
            // Generate a random value in the range [low, high).
            // I suppose we need integer values in the range [1, 6]
            roll[i] =  rng.gen_range(MIN_DICE_NUMBER, MAX_DICE_NUMBER + 1);
        }
    }
    roll
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
        // add coins sent by a second player
        self.game.game_pool.joined_player_pool = locked_per_player(&self.game.base_bet);

        self.game.joined_player_address = joined_player_address;
        self.game.joined_player_nft_id = joined_player_nft_id;
        self.joined_player_secret = joined_player_secret;

        // game started
        self.game.status = GameStatus::Started;
    }

    /// Roll dices
    pub fn roll(&mut self, game_id: GameId) {
        let mut combined_secret = self.host_player_secret.to_vec();
        combined_secret.extend(self.joined_player_secret);
        combined_secret.extend(&game_id.to_be_bytes()); // game counter

        let seed: [u8; 32] = Sha256::digest(&combined_secret).into();

        let mut rng = ChaChaRng::from_seed(seed);

        let mut roll: [u8; NUM_OF_DICES] = [0; NUM_OF_DICES];

        // Generate a random value in the range [low, high).
        // I suppose we need integer values in the range [1, 6]
        for dice in &mut roll {
            *dice = rng.gen_range(MIN_DICE_NUMBER, MAX_DICE_NUMBER + 1);
        }

        // Change roll turn value
        match self.game.roll_turn {
            Player::Host => {
                self.game.host_player_rolls[0] = Some(roll);
                self.game.host_player_total_points = calculate_player_total_points(roll);
                self.game.roll_turn = Player::Joined;
            }
            Player::Joined => {
                self.game.joined_player_rolls[0] = Some(roll);
                self.game.joined_player_total_points = calculate_player_total_points(roll);
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
    pub fn reroll(&mut self, game_id: GameId, dices: [bool; NUM_OF_DICES]) {
        let mut combined_secret = self.joined_player_secret.to_vec();
        combined_secret.extend(self.host_player_secret);
        combined_secret.extend(&game_id.to_be_bytes()); // game counter

        let seed: [u8; 32] = Sha256::digest(&combined_secret).into();

        let mut rng = ChaChaRng::from_seed(seed);

        match self.game.roll_turn {
            Player::Host => {

                if let Some(roll) = self.game.host_player_rolls[0] { 

                    // no dices to reroll
                    if dices.iter().all(|dice| *dice == false) {
                        self.game.host_player_rolls[1] = Some(roll);
                    } else {
                        // reroll chosen dices
                        let reroll = complete_reroll(&mut rng, roll, dices);
                        self.game.host_player_rolls[1] = Some(reroll);
                        self.game.host_player_total_points = calculate_player_total_points(reroll);
                    }
                }

                self.game.roll_turn = Player::Joined;
            }
            Player::Joined => {

                if let Some(roll) = self.game.joined_player_rolls[0] { 
                    // no dices to reroll
                    if dices.iter().all(|dice| *dice == false) {
                        self.game.joined_player_rolls[1] = Some(roll);
                    } else {
                        // reroll chosen dices
                        let reroll = complete_reroll(&mut rng, roll, dices);
                        self.game.joined_player_rolls[1] = Some(reroll);
                        self.game.joined_player_total_points = calculate_player_total_points(reroll);
                    }
                }

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
    pub fn ensure_can_roll(&self, address: CanonicalAddr) -> ContractResult<()> {
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
    // game pool
    pub game_pool: GamePool,
    // host player pool
    pub host_player_pool: Coin,
    // host player pool
    pub joined_player_pool: Coin,
    // total points amount scored throughout the game by host player
    pub host_player_total_points: u8,
    // total points amount scored throughout the game by joined player
    pub joined_player_total_points: u8,

    // who rolls next (default initial player is set to host)
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
            game_pool: GamePool::new(locked_per_player(&base_bet)),
            base_bet,
            ..Game::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub struct GamePool {
    // an amount already staked in the game
    total_stake: Coin,
    // host player pool
    host_player_pool: Coin,
    // joined player pool
    joined_player_pool: Coin,
}

impl GamePool {
    /// Create a new GamePool
    fn new(host_player_pool: Coin) -> Self {
        Self {
            total_stake: Coin::default(),
            host_player_pool,
            joined_player_pool: Coin::default(),
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
