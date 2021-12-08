use serde::{de::DeserializeOwned, Serialize};
use std::any::type_name;

use crate::contract::GameId;
use crate::game::GameDetails;

use cosmwasm_std::{CanonicalAddr, HumanAddr, ReadonlyStorage, StdError, StdResult, Storage};

use secret_toolkit::serialization::{Bincode2, Json, Serde};

pub static CONFIG_KEY: &[u8] = b"config";

/// prefix for the games
pub const PREFIX_GAMES: &[u8] = b"games";

/// prefix for the last created game id
pub const PREFIX_LAST_GAME_INDEX: &[u8] = b"gameId";

/// prefix for the nft contract address
pub const PREFIX_NFT_CONTRACT: &[u8] = b"nftContract";

/// prefix for the nft code id
pub const PREFIX_NFT_CODE_ID: &[u8] = b"nftCodeId";

/// prefix for the nft code hash
pub const PREFIX_NFT_CODE_HASH: &[u8] = b"nftCodeHash";

/// prefix for the games
pub const PREFIX_ADMIN: &[u8] = b"admin";

// last game index
pub fn save_last_game_index<S: Storage>(storage: &mut S, index: &GameId) -> StdResult<()> {
    save(storage, PREFIX_LAST_GAME_INDEX, index)
}

// readonly last game index
pub fn load_last_game_index<S: Storage>(storage: &S) -> StdResult<GameId> {
    load(storage, PREFIX_LAST_GAME_INDEX)
}

// admin
pub fn save_admin<S: Storage>(storage: &mut S, admin: &HumanAddr) -> StdResult<()> {
    save(storage, PREFIX_ADMIN, admin)
}

// readonly last game index
pub fn load_admin<S: Storage>(storage: &S) -> StdResult<HumanAddr> {
    load(storage, PREFIX_ADMIN)
}
// supporting nft contract
// currently only 1
pub fn save_nft_address<S: Storage>(storage: &mut S, nft_address: &HumanAddr) -> StdResult<()> {
    save(storage, PREFIX_NFT_CONTRACT, nft_address)
}

pub fn nft_address<S: Storage>(storage: &S) -> StdResult<HumanAddr> {
    load(storage, PREFIX_NFT_CONTRACT)
}

pub fn save_nft_code_id<S: Storage>(storage: &mut S, id: u64) -> StdResult<()> {
    save(storage, PREFIX_NFT_CODE_ID, &id)
}

pub fn nft_code_id<S: Storage>(storage: &S) -> StdResult<u64> {
    load(storage, PREFIX_NFT_CODE_ID)
}

pub fn save_nft_code_hash<S: Storage>(storage: &mut S, hash: String) -> StdResult<()> {
    save(storage, PREFIX_NFT_CODE_HASH, &hash)
}

pub fn nft_code_hash<S: Storage>(storage: &S) -> StdResult<String> {
    load(storage, PREFIX_NFT_CODE_HASH)
}

// Get game storage key from it's id
pub fn get_game_key(game_id: GameId) -> Vec<u8> {
    PREFIX_GAMES
        .iter()
        .chain(game_id.to_be_bytes().iter())
        .copied()
        .collect()
}

pub fn save_joiner<S: Storage>(
    storage: &mut S,
    joiner: &CanonicalAddr,
    viewing_key: String,
) -> StdResult<()> {
    save(storage, &joiner.0 .0, &viewing_key)
}

pub fn load_joiner<S: Storage>(storage: &S, joiner: &CanonicalAddr) -> StdResult<Option<String>> {
    may_load(storage, &joiner.0 .0)
}

pub fn save_game<S: Storage>(
    storage: &mut S,
    game_id: GameId,
    value: &GameDetails,
) -> StdResult<()> {
    let key: Vec<u8> = get_game_key(game_id);
    json_save(storage, &key, value)
}

pub fn load_game<S: Storage>(storage: &S, game_id: GameId) -> StdResult<GameDetails> {
    let key: Vec<u8> = get_game_key(game_id);
    json_load(storage, &key)
}

pub fn remove_game<S: Storage>(storage: &mut S, game_id: GameId) {
    let key: Vec<u8> = get_game_key(game_id);
    remove(storage, &key)
}

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

/// Removes an item from storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

/// Returns StdResult<T> from retrieving the item with the specified key.  Returns a
/// StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

/// Returns StdResult<Option<T>> from retrieving the item with the specified key.
/// Returns Ok(None) if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}

/// Returns StdResult<()> resulting from saving an item to storage using Json (de)serialization
/// because bincode2 annoyingly uses a float op when deserializing an enum
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn json_save<T: Serialize, S: Storage>(
    storage: &mut S,
    key: &[u8],
    value: &T,
) -> StdResult<()> {
    storage.set(key, &Json::serialize(value)?);
    Ok(())
}

/// Returns StdResult<T> from retrieving the item with the specified key using Json
/// (de)serialization because bincode2 annoyingly uses a float op when deserializing an enum.
/// Returns a StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn json_load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Json::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

/// Returns StdResult<Option<T>> from retrieving the item with the specified key using Json
/// (de)serialization because bincode2 annoyingly uses a float op when deserializing an enum.
/// Returns Ok(None) if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn json_may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Json::deserialize(&value).map(Some),
        None => Ok(None),
    }
}
