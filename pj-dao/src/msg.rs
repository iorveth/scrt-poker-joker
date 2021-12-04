use crate::game::GameDetails;
use cosmwasm_std::{Api, Binary, Coin, HumanAddr, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::game::GameStatus;
use crate::game::NUM_OF_DICES;

use crate::contract::{GameId, Secret};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub nft_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateNFTContract {
        code_id: u64,
    },
    StoreNFTContract {},
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
    // NFT address
    NftAddress {},
}

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct GameResonse(GameDetails);

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct ActiveGamesResponse(Vec<GameDetails>);

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct PendingGamesResponse(Vec<GameDetails>);

// ----- From NFT contract, todo move to lib ------
/// NFT Instantiation message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NftInitMsg {
    /// name of token contract
    pub name: String,
    /// token contract symbol
    pub symbol: String,
    /// optional admin address, env.message.sender if missing
    pub admin: Option<HumanAddr>,
    /// entropy used for prng seed
    pub entropy: String,
    /// optional royalty information to use as default when RoyaltyInfo is not provided to a
    /// minting function
    pub royalty_info: Option<RoyaltyInfo>,
    /// optional privacy configuration for the contract
    pub config: Option<InitConfig>,
    /// optional callback message to execute after instantiation.  This will
    /// most often be used to have the token contract provide its address to a
    /// contract that instantiated it, but it could be used to execute any
    /// contract
    pub post_init_callback: Option<PostInitCallback>,
}

/// info needed to perform a callback message after instantiation
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct PostInitCallback {
    /// the callback message to execute
    pub msg: Binary,
    /// address of the contract to execute
    pub contract_address: HumanAddr,
    /// code hash of the contract to execute
    pub code_hash: String,
    /// list of native Coin to send with the callback message
    pub send: Vec<Coin>,
}

/// data for a single royalty
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Royalty {
    /// address to send royalties to
    pub recipient: HumanAddr,
    /// royalty rate
    pub rate: u16,
}

/// all royalty information
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RoyaltyInfo {
    /// decimal places in royalty rates
    pub decimal_places_in_rates: u8,
    /// list of royalties
    pub royalties: Vec<Royalty>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InitConfig {
    /// indicates whether the token IDs and the number of tokens controlled by the contract are
    /// public.  If the token supply is private, only minters can view the token IDs and
    /// number of tokens controlled by the contract
    /// default: False
    pub public_token_supply: Option<bool>,
    /// indicates whether token ownership is public or private.  A user can still change whether the
    /// ownership of their tokens is public or private
    /// default: False
    pub public_owner: Option<bool>,
    /// indicates whether sealed metadata should be enabled.  If sealed metadata is enabled, the
    /// private metadata is not viewable by anyone, not even the owner, until the owner calls the
    /// Reveal function.  When Reveal is called, the sealed metadata is irreversibly moved to the
    /// public metadata (as default).  if unwrapped_metadata_is_private is set to true, it will
    /// remain as private metadata, but the owner will now be able to see it.  Anyone will be able
    /// to query the token to know that it has been unwrapped.  This simulates buying/selling a
    /// wrapped card that no one knows which card it is until it is unwrapped. If sealed metadata
    /// is not enabled, all tokens are considered unwrapped
    /// default:  False
    pub enable_sealed_metadata: Option<bool>,
    /// indicates if the Reveal function should keep the sealed metadata private after unwrapping
    /// This config value is ignored if sealed metadata is not enabled
    /// default: False
    pub unwrapped_metadata_is_private: Option<bool>,
    /// indicates whether a minter is permitted to update a token's metadata
    /// default: True
    pub minter_may_update_metadata: Option<bool>,
    /// indicates whether the owner of a token is permitted to update a token's metadata
    /// default: False
    pub owner_may_update_metadata: Option<bool>,
    /// Indicates whether burn functionality should be enabled
    /// default: False
    pub enable_burn: Option<bool>,
}
// ----- From NFT contract, todo move to lib ------
/// info needed to perform a callback message after instantiation
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct NftPostInitCallback {
    /// the callback message to execute
    pub msg: Binary,
    /// address of the contract to execute
    pub contract_address: HumanAddr,
    /// code hash of the contract to execute
    pub code_hash: String,
    /// list of native Coin to send with the callback message
    pub send: Vec<Coin>,
}
