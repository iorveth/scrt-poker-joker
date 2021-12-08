use crate::error::{ContractError, ContractResult};
use crate::game::{GameDetails, GameStatus, NUM_OF_DICES};
use cosmwasm_std::{Binary, Coin, HumanAddr, StdError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{GameId, Secret};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub nft_code_id: u64,
    pub nft_code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateNftContract {},
    StoreNftContract {},
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
    JoinDao {
        nft: Option<JoinNftDetails>,
    },
    AdminMint {
        to: HumanAddr,
        private_metadata: Option<Metadata>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // retrieve all games by status provided
    GamesByStatus {
        status: GameStatus,
    },
    // get game under specified id
    Game {
        game_id: GameId,
    },
    // NFT address
    NftAddress {},
    // retrieve Nfts from player
    PlayerNfts {
        player: HumanAddr,
        viewer: HumanAddr,
    },
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

// ---- specific messages for the NFT contract ----

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftHandleMsg {
    MintDiceNft {
        owner: HumanAddr,
        /// viewing key set by the dao for this dice nft
        key: String,
        /// optional public metadata that can be seen by everyone
        private_metadata: Option<Metadata>,
    },
    SetMetadata {
        /// id of the token whose metadata should be updated
        token_id: String,
        /// the optional new public metadata
        public_metadata: Option<Metadata>,
        /// the optional new private metadata
        private_metadata: Option<Metadata>,
        /// optional message length padding
        padding: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JoinNftDetails {
    pub id: String,
    pub viewing_key: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NftQueryAnswer {
    OwnerOf {
        owner: HumanAddr,
        approvals: Vec<Cw721Approval>,
    },
    TokenList {
        tokens: Vec<String>,
    },
    NftInfo {
        token_uri: Option<String>,
        extension: Option<Extension>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NftQueryMsg {
    OwnerOf {
        token_id: String,
        /// optional address and key requesting to view the token owner
        viewer: Option<ViewerInfo>,
        /// optionally include expired Approvals in the response list.  If ommitted or
        /// false, expired Approvals will be filtered out of the response
        include_expired: Option<bool>,
    },
    /// displays the public metadata of a token
    NftInfo { token_id: String },
    /// displays a list of all the tokens belonging to the input owner in which the viewer
    /// has view_owner permission
    Tokens {
        owner: HumanAddr,
        /// optional address of the querier if different from the owner
        viewer: Option<HumanAddr>,
        /// optional viewing key
        viewing_key: Option<String>,
        /// paginate by providing the last token_id received in the previous query
        start_after: Option<String>,
        /// optional number of token ids to display
        limit: Option<u32>,
    },
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

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Metadata {
    /// optional uri for off-chain metadata.  This should be prefixed with `http://`, `https://`, `ipfs://`, or
    /// `ar://`.  Only use this if you are not using `extension`
    pub token_uri: Option<String>,
    /// optional on-chain metadata.  Only use this if you are not using `token_uri`
    pub extension: Option<Extension>,
}

/// metadata extension
/// You can add any metadata fields you need here.  These fields are based on
/// https://docs.opensea.io/docs/metadata-standards and are the metadata fields that
/// Stashh uses for robust NFT display.  Urls should be prefixed with `http://`, `https://`, `ipfs://`, or
/// `ar://`
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Extension {
    /// url to the image
    pub image: Option<String>,
    /// raw SVG image data (not recommended). Only use this if you're not including the image parameter
    pub image_data: Option<String>,
    /// url to allow users to view the item on your site
    pub external_url: Option<String>,
    /// item description
    pub description: Option<String>,
    /// dice experience level
    pub xp: u32,
    /// name of the item
    pub name: Option<String>,
    /// item attributes
    pub attributes: Vec<Trait>,
    /// background color represented as a six-character hexadecimal without a pre-pended #
    pub background_color: Option<String>,
    /// url to a multimedia attachment
    pub animation_url: Option<String>,
    /// url to a YouTube video
    pub youtube_url: Option<String>,
    /// media files as specified on Stashh that allows for basic authenticatiion and decryption keys.
    /// Most of the above is used for bridging public eth NFT metadata easily, whereas `media` will be used
    /// when minting NFTs on Stashh
    pub media: Option<Vec<MediaFile>>,
    /// a select list of trait_types that are in the private metadata.  This will only ever be used
    /// in public metadata
    pub protected_attributes: Option<Vec<String>>,
}

impl Extension {
    /// Ensure NFT has enough XP for a game with a given base bet
    pub fn ensure_enough_xp_for_the_base_bet(&self, base_bet: &Coin) -> ContractResult<()> {
        let base_bet_amount = base_bet.amount.u128();

        let enough_xp = match self.xp {
            0..=10 => base_bet_amount == 1,
            11..=20 => base_bet_amount <= 2,
            21..=40 => base_bet_amount <= 4,
            _ => base_bet_amount <= 8,
        };

        if !enough_xp {
            Err(StdError::generic_err(
                ContractError::NotEnoughXpForTheBaseBet {}.to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

/// attribute trait
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Trait {
    /// indicates how a trait should be displayed
    pub display_type: Option<String>,
    /// name of the trait
    pub trait_type: Option<String>,
    /// trait value
    pub value: String,
    /// optional max value for numerical traits
    pub max_value: Option<String>,
}

/// media file
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct MediaFile {
    /// file type
    /// Stashh currently uses: "image", "video", "audio", "text", "font", "application"
    pub file_type: Option<String>,
    /// file extension
    pub extension: Option<String>,
    /// authentication information
    pub authentication: Option<Authentication>,
    /// url to the file.  Urls should be prefixed with `http://`, `https://`, `ipfs://`, or `ar://`
    pub url: String,
}

/// media file authentication
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Authentication {
    /// either a decryption key for encrypted files or a password for basic authentication
    pub key: Option<String>,
    /// username used in basic authentication
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum Expiration {
    /// expires at this block height
    AtHeight(u64),
    /// expires at the time in seconds since 01/01/1970
    AtTime(u64),
    /// never expires
    Never,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Cw721Approval {
    pub spender: HumanAddr,
    pub expires: Expiration,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ViewerInfo {
    /// querying address
    pub address: HumanAddr,
    /// authentication key string
    pub viewing_key: String,
}
