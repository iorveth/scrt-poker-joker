use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;

use crate::contract::GameId;
use crate::game::GameDetails;

use cosmwasm_std::{Api, BlockInfo, CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, Bucket, PrefixedStorage, ReadonlyBucket,
    ReadonlyPrefixedStorage, Singleton,
};

use secret_toolkit::{
    serialization::{Bincode2, Json, Serde},
    storage::{AppendStore, AppendStoreMut},
};

pub static CONFIG_KEY: &[u8] = b"config";

/// prefix for the games
pub const PREFIX_GAMES: &[u8] = b"games";

/// prefix for the last created game id
pub const PREFIX_LAST_GAME_INDEX: &[u8] = b"gameId";

pub fn games_mut<'a, S: Storage>(storage: &'a mut S) -> Bucket<'a, S, GameDetails> {
    bucket(PREFIX_GAMES, storage)
}

pub fn games<'a, S: Storage>(storage: &'a S) -> ReadonlyBucket<'a, S, GameDetails> {
    bucket_read(PREFIX_GAMES, storage)
}

// retrieve last game index
pub fn last_game_index<'a, S: Storage>(storage: &'a mut S) -> Singleton<'a, S, GameId> {
    singleton(storage, PREFIX_LAST_GAME_INDEX)
}
