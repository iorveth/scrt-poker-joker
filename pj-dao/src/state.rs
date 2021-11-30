use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;

use cosmwasm_std::{Api, BlockInfo, CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use secret_toolkit::{
    serialization::{Bincode2, Json, Serde},
    storage::{AppendStore, AppendStoreMut},
};

pub static CONFIG_KEY: &[u8] = b"config";
/// prefix for the storage of all Game IDs
pub const PREFIX_GAMES_INDEX: &[u8] = b"gameIds";
/// prefix for the storage of all Active Games
pub const PREFIX_ACITVE_GAMES: &[u8] = b"activegame";
/// prefix for the storage of all Pending Games
pub const PREFIX_PENDING_GAMES: &[u8] = b"pendinggame";
/// prefix for the storage of all Joiners
pub const PREFIX_JOINERS: &[u8] = b"joiners";

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    let b = Bincode2::serialize(value).map_err(|_| StdError::generic_err("serialisation"));
    storage.set(key, &b?);
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
    let de = &storage
        .get(key)
        .ok_or_else(|| StdError::not_found(type_name::<T>()))?;
    Bincode2::deserialize(de).map_err(|e| StdError::generic_err(e.to_string()))
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
        Some(value) => Bincode2::deserialize(&value)
            .map(Some)
            .map_err(|e| StdError::generic_err(e.to_string())),
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
    let v = Json::serialize(value).map_err(|e| StdError::generic_err(e.to_string()));
    storage.set(key, &v?);
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
    let de = &storage
        .get(key)
        .ok_or_else(|| StdError::not_found(type_name::<T>()))?;
    Json::deserialize(de).map_err(|_| StdError::not_found("storage"))?
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
        Some(value) => Json::deserialize(&value)
            .map(Some)
            .map_err(|e| StdError::generic_err(e.to_string())),
        None => Ok(None),
    }
}
