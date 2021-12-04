use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;

use crate::contract::GameId;
use crate::game::GameDetails;

use cosmwasm_std::{
    Api, BlockInfo, CanonicalAddr, HumanAddr, ReadonlyStorage, StdError, StdResult, Storage,
};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, PrefixedStorage, ReadonlyBucket,
    ReadonlyPrefixedStorage, ReadonlySingleton, Singleton,
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

/// prefix for the nft contract address
pub const PREFIX_NFT_CONTRACT: &[u8] = b"nftContract";

/// prefix for the nft code id
pub const PREFIX_NFT_CODE_ID: &[u8] = b"nftCodeId";

// There is issue with using certain buckets and singletone for wasm, the contract does not run on the secret
// network node.
// if you go to scrt discord and search db_scan you will find the conversation I have pasted below:
//
// IIRC I think it fails because SecretNetwork doesn't implement db_scan
// And I think I was told that if it was implemented, all the keys you get with your key-value pairs from range would be encrypted anyway, but that was back during phase 2 testnet, so my memory might be off
// Assaf | SCRT Labs — 27/11/2020
// Yep we decided db_scan is very hard to implement in a secure way. We felt that the use case for it is thin at best and went ahead with the mainnet release.
// I'd recommend though to use bincode2 for storage serialization (more efficient) and serde_json_wasm for query result (better ux)

pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(PREFIX_GAMES, &Bincode2::serialize(value)?);
    Ok(())
}

pub fn games_mut<'a, S: Storage>(storage: &'a mut S) -> Bucket<'a, S, GameDetails> {
    bucket(PREFIX_GAMES, storage)
}

pub fn games<'a, S: Storage>(storage: &'a S) -> ReadonlyBucket<'a, S, GameDetails> {
    bucket_read(PREFIX_GAMES, storage)
}

// last game index
pub fn last_game_index_mut<'a, S: Storage>(storage: &'a mut S) -> Singleton<'a, S, GameId> {
    singleton(storage, PREFIX_LAST_GAME_INDEX)
}

// readonly last game index
pub fn last_game_index<'a, S: Storage>(storage: &'a S) -> ReadonlySingleton<'a, S, GameId> {
    singleton_read(storage, PREFIX_LAST_GAME_INDEX)
}
}
// supporting nft contract
// currently only 1
pub fn nft_address_mut<'a, S: Storage>(storage: &'a mut S) -> Singleton<'a, S, HumanAddr> {
    singleton(storage, PREFIX_NFT_CONTRACT)
}
pub fn nft_address<'a, S: Storage>(storage: &'a S) -> ReadonlySingleton<'a, S, HumanAddr> {
    singleton_read(storage, PREFIX_NFT_CONTRACT)
}

// supporting nft code id
pub fn nft_code_id<'a, S: Storage>(storage: &'a mut S) -> Singleton<'a, S, u64> {
    singleton(storage, PREFIX_NFT_CODE_ID)
}
