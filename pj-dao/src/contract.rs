use crate::error::{ContractError, ContractResult};
use crate::game::{locked_per_player, Game, GameDetails, GameStatus, Player, NUM_OF_DICES};
use crate::msg::{
    HandleMsg, InitMsg, JoinNftDetails, Metadata, NftHandleMsg, NftInitMsg, NftQueryAnswer,
    NftQueryMsg, PostInitCallback, QueryMsg, QueryWithPermit,
};
use crate::state::{
    load_admin, load_game, load_joiner, load_last_game_index, nft_address, nft_code_hash,
    nft_code_id, remove_game, save_admin, save_game, save_joiner, save_last_game_index,
    save_nft_address, save_nft_code_hash, save_nft_code_id,
};
use cosmwasm_std::{
    has_coins, log, to_binary, Api, Binary, CanonicalAddr, Coin, CosmosMsg, Env, Extern,
    HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, StdError, StdResult, Storage,
    WasmMsg, WasmQuery,
};
use secret_toolkit::permit::Permit;
use secret_toolkit::serialization::{Json, Serde};

pub type GameId = u64;

pub type Secret = u64;

/// Initial game index
pub const INIT_INDEX: GameId = 0;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    save_last_game_index(&mut deps.storage, &INIT_INDEX)?;
    save_nft_code_hash(&mut deps.storage, msg.nft_code_hash)?;
    save_nft_code_id(&mut deps.storage, msg.nft_code_id)?;
    save_admin(&mut deps.storage, &env.message.sender)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> ContractResult<HandleResponse> {
    match msg {
        HandleMsg::CreateNftContract {} => create_nft_contract(deps, env),
        HandleMsg::StoreNftContract {} => store_nft_contract_addr(deps, env),
        HandleMsg::JoinDao { nft } => join_dao(deps, env, nft),
        HandleMsg::CreateNewGameRoom {
            nft_id,
            base_bet,
            secret,
            permit,
        } => create_new_game_room(deps, env, nft_id, base_bet, secret, permit),
        HandleMsg::JoinGame {
            nft_id,
            game_id,
            secret,
            permit,
        } => join_game(deps, env, nft_id, game_id, secret, permit),
        HandleMsg::Roll { game_id } => roll(deps, env, game_id),
        HandleMsg::ReRoll { game_id, dices } => reroll(deps, env, game_id, dices),
        HandleMsg::AdminMint {
            to,
            private_metadata,
        } => admin_mint(deps, env, to, private_metadata),
        HandleMsg::EndGame { game_id } => end_game(deps, env, game_id),
    }
}

pub fn admin_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    to: HumanAddr,
    private_metadata: Option<Metadata>,
) -> ContractResult<HandleResponse> {
    ensure_is_admin(deps, &env.message.sender)?;
    let msg = mint_dice_nft_handle_msg(&to, private_metadata);

    save_joiner(&mut deps.storage, &deps.api.canonical_address(&to)?)?;

    let contract_addr = nft_address(&deps.storage)?;
    let callback_code_hash = nft_code_hash(&deps.storage)?;
    let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr,
        callback_code_hash,
        msg: to_binary(&msg)?,
        send: vec![],
    });

    Ok(HandleResponse {
        messages: vec![mint_msg],
        log: vec![log("minted for: ", env.message.sender)],
        data: None,
    })
}

pub fn join_dao<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    nft: Option<JoinNftDetails>,
) -> ContractResult<HandleResponse> {
    let player_raw = deps.api.canonical_address(&env.message.sender)?;

    // Ensure user is not a dao member yet
    ensure_is_not_a_dao_member(&deps.storage, &player_raw)?;

    // TODO: charge user?

    let mut response_msg: Vec<CosmosMsg> = vec![];

    // if a nft_id is provided, we check if msg.sender owns the nft
    // in order to check, dao must be provided the Permit
    if let Some(nft) = nft {
        let owner_of_msg = QueryWithPermit::OwnerOf {
            token_id: nft.nft_id,
            include_expired: None,
        };

        let query = WasmQuery::Smart {
            contract_addr: nft_address(&deps.storage)?,
            callback_code_hash: nft_code_hash(&deps.storage)?,
            msg: to_binary(&to_permit_msg(nft.permit, owner_of_msg))?,
        };
        let result: NftQueryAnswer = deps.querier.query(&QueryRequest::Wasm(query))?;
        let returned_owner = match result {
            NftQueryAnswer::OwnerOf {
                owner,
                approvals: _,
            } => owner,
            _ => return Err(StdError::generic_err("Error parsing query results")),
        };

        if env.message.sender != returned_owner {
            return Err(StdError::generic_err(
                "Not authorised to join with this nft",
            ));
        }
        save_joiner(&mut deps.storage, &player_raw)?;
    } else {
        // we will mint a new nft for the owner
        let msg = mint_dice_nft_handle_msg(&env.message.sender, None);

        // save the new joiner
        save_joiner(&mut deps.storage, &player_raw)?;

        let contract_addr = nft_address(&deps.storage)?;
        let callback_code_hash = nft_code_hash(&deps.storage)?;
        let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            callback_code_hash,
            msg: to_binary(&msg)?,
            send: vec![],
        });
        response_msg.push(mint_msg);
    }
    Ok(HandleResponse {
        messages: response_msg,
        log: vec![log("member joined dao", env.message.sender)],
        data: None,
    })
}

pub fn store_nft_contract_addr<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> ContractResult<HandleResponse> {
    save_nft_address(&mut deps.storage, &env.message.sender)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("nft address saved", env.message.sender)],
        data: None,
    })
}

pub fn create_nft_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> ContractResult<HandleResponse> {
    if nft_address(&deps.storage).is_ok() {
        return Err(StdError::generic_err(
            ContractError::AlreadyHasNFTContract {}.to_string(),
        ));
    }
    let code_id = nft_code_id(&deps.storage)?;
    let callback_code_hash = nft_code_hash(&deps.storage)?;
    let admin = load_admin(&deps.storage)?;
    let store_addr_msg = HandleMsg::StoreNftContract {};
    let post_init_callback = PostInitCallback {
        msg: to_binary(&store_addr_msg)?,
        contract_address: env.contract.address.clone(),
        code_hash: env.contract_code_hash,
        send: vec![],
    };
    let nft_instantiate_msg = NftInitMsg {
        name: "PokerJokerNFT".into(),
        symbol: "PJX".into(),
        admin: Some(admin),
        entropy: "HACKATOM IV".into(),
        royalty_info: None,
        config: None,
        post_init_callback: Some(post_init_callback),
    };
    let instantiate_msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        code_id,
        send: vec![],
        callback_code_hash,
        msg: to_binary(&nft_instantiate_msg)?,
        label: "PJDAO-NFT".into(),
    });

    Ok(HandleResponse {
        messages: vec![instantiate_msg],
        log: vec![log("nft contract created", code_id)],
        data: None,
    })
}

pub fn create_new_game_room<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    nft_id: String,
    base_bet: Coin,
    secret: Secret,
    permit: Permit,
) -> ContractResult<HandleResponse> {
    // Ensure given account joined dao, retrieve it's nfts.
    let player_nfts = query_player_nfts(deps, &env.message.sender, permit)?;

    // Ensure given nft belongs to player
    ensure_can_access_nft(player_nfts, &nft_id)?;

    // Ensure player can use given nft in a game
    ensure_can_use_nft_in_a_game(deps, nft_id.clone(), &base_bet)?;

    // check whether nft supports base_bet

    // ensure base bet is greater then zero
    ensure_correct_base_bet(&base_bet)?;

    // ensure enough coins provided
    ensure_has_coins_for_game(&env, &base_bet)?;

    let game_id = load_last_game_index(&deps.storage)?;

    // create new game with provided host player secret
    let game = Game::new(env.message.sender, nft_id, base_bet);
    let game_details = GameDetails::new(game, secret.to_be_bytes());

    // save newly initialized game
    save_game(&mut deps.storage, game_id, &game_details)?;

    // increment game index
    save_last_game_index(&mut deps.storage, &(game_id + 1))?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("game room created, id: ", game_id)],
        data: None,
    })
}

pub fn join_game<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    nft_id: String,
    game_id: GameId,
    secret: Secret,
    permit: Permit,
) -> ContractResult<HandleResponse> {
    // Ensure given account joined dao, retrieve it's nfts.
    let player_nfts = query_player_nfts(deps, &env.message.sender, permit)?;

    // Ensure given nft belongs to player
    ensure_can_access_nft(player_nfts, &nft_id)?;

    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    let base_bet = game_details.game.base_bet.clone();

    // Ensure player can use given nft in a game
    ensure_can_use_nft_in_a_game(deps, nft_id.clone(), &base_bet)?;

    // ensure enough coins provided
    ensure_has_coins_for_game(&env, &base_bet)?;

    // ensure game status is set to pending
    game_details.ensure_is_pending()?;

    // join the game
    game_details.join(env.message.sender, nft_id, secret.to_be_bytes());

    // save updated game state
    save_game(&mut deps.storage, game_id, &game_details)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("joined the game", game_id)],
        data: None,
    })
}

pub fn roll<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    game_id: GameId,
) -> ContractResult<HandleResponse> {
    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    // ensure game status is set to started
    game_details.ensure_is_started()?;

    // Ensure given account can now make a roll in a game
    game_details.ensure_can_roll(env.message.sender)?;

    game_details.roll(game_id);

    // save updated game state
    save_game(&mut deps.storage, game_id, &game_details)?;

    let game_json = Json::serialize(&Game::from(game_details))?;

    // check whether player can roll
    Ok(HandleResponse {
        messages: vec![],
        log: vec![log(
            "rolled",
            format!("game_id {} \n {:?} ", game_id, game_json),
        )],
        data: None,
    })
}

pub fn reroll<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    game_id: GameId,
    dices: [bool; NUM_OF_DICES],
) -> ContractResult<HandleResponse> {
    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    // ensure game status is set to reroll
    game_details.ensure_is_reroll()?;

    // Ensure given account can make a reroll in a game
    game_details.ensure_can_roll(env.message.sender)?;

    game_details.reroll(game_id, dices);

    let game_json = Json::serialize(&Game::from(game_details.clone()))?;

    // save updated game state
    save_game(&mut deps.storage, game_id, &game_details)?;

    let log = vec![log(
        "rerolled",
        format!("game_id {} \n {:?} ", game_id, game_json),
    )];

    Ok(HandleResponse {
        messages: vec![],
        log,
        data: None,
    })
}

pub fn end_game<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    game_id: GameId,
) -> ContractResult<HandleResponse> {
    // ensure game exists
    let game_details = load_game(&deps.storage, game_id)?;

    // ensure game is finished and can be ended
    game_details.ensure_is_finished()?;

    // determine a winner and complete payments
    let winner = game_details.determine_a_winner();

    // Ensure actor can complete a game
    game_details.ensure_can_complete_a_game(env.message.sender, winner)?;

    let game_json = Json::serialize(&Game::from(game_details.clone()))?;

    // we need to increase nft xp if there is a winner
    let set_nft_metadata_msg = if let Some(winner) = winner {
        Some(get_set_nft_metadata_msg(deps, &game_details, winner)?)
    } else {
        None
    };

    let mut messages = game_details.complete_checkout(env.contract.address, winner);

    if let Some(msg) = set_nft_metadata_msg {
        messages.push(msg);
    }

    // remove game after completion
    remove_game(&mut deps.storage, game_id);

    let log = vec![log(
        "game completed",
        format!("game_id {} \n {:?} ", game_id, game_json),
    )];

    Ok(HandleResponse {
        messages,
        log,
        data: None,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Game { game_id } => to_binary(&query_game(deps, game_id)?),
        QueryMsg::GamesByStatus { status } => to_binary(&query_games_by_status(deps, status)?),
        QueryMsg::NftAddress {} => to_binary(&query_nft_address(deps)?),
        QueryMsg::PlayerNfts { player, permit } => {
            to_binary(&query_player_nfts(deps, &player, permit)?)
        }
        QueryMsg::NftInfo { token_id } => to_binary(&query_nft_info_by_id(deps, token_id)?),
    }
}

// query game by it's id
fn query_game<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    game_id: GameId,
) -> StdResult<Game> {
    load_game(&deps.storage, game_id).map(Game::from)
}

fn query_nft_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<HumanAddr> {
    nft_address(&deps.storage)
}

// returns a vector of [u8] game keys and their details
fn query_games_by_status<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    status: GameStatus,
) -> StdResult<Vec<(GameId, Game)>> {
    (INIT_INDEX..=load_last_game_index(&deps.storage)?)
        .into_iter()
        .map(|i| match load_game(&deps.storage, i) {
            Ok(game) => Ok((i, Game::from(game))),
            Err(e) => Err(e),
        })
        .filter(|game_entry| {
            if let Ok((_, game)) = game_entry {
                game.status == status
            } else {
                false
            }
        })
        .collect()
}

pub fn to_permit_msg(permit: Permit, query: QueryWithPermit) -> NftQueryMsg {
    NftQueryMsg::WithPermit { permit, query }
}

/// Query all the player nfts by the provided viewer key
fn query_player_nfts<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    player: &HumanAddr,
    permit: Permit,
) -> StdResult<Vec<String>> {
    // Ensure given account joined dao
    ensure_is_dao_member(deps, player)?;

    let permit_query = to_permit_msg(
        permit,
        QueryWithPermit::Tokens {
            owner: player.to_owned(),
            /// paginate by providing the last token_id received in the previous query
            start_after: None,
            /// optional number of token ids to display
            limit: None,
        },
    );

    let tokens: NftQueryAnswer = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_address(&deps.storage)?,
        /// callback_code_hash is the hex encoded hash of the code. This is used by Secret Network to harden against replaying the contract
        /// It is used to bind the request to a destination contract in a stronger way than just the contract address which can be faked
        callback_code_hash: nft_code_hash(&deps.storage)?,
        /// msg is the json-encoded QueryMsg struct
        msg: to_binary(&permit_query)?,
    }))?;

    match tokens {
        NftQueryAnswer::TokenList { tokens: list } => Ok(list),
        _ => Err(StdError::generic_err(
            ContractError::QueryPlayerNotValid {}.to_string(),
        )),
    }
}

/// Query nft info by it's id
fn query_nft_info_by_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    token_id: String,
) -> StdResult<NftQueryAnswer> {
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_address(&deps.storage)?,
        /// callback_code_hash is the hex encoded hash of the code. This is used by Secret Network to harden against replaying the contract
        /// It is used to bind the request to a destination contract in a stronger way than just the contract address which can be faked
        callback_code_hash: nft_code_hash(&deps.storage)?,
        /// msg is the json-encoded QueryMsg struct
        msg: to_binary(&NftQueryMsg::NftInfo { token_id })?,
    }))
}

/// Ensure given account is contract admin
fn ensure_is_admin<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    sender: &HumanAddr,
) -> ContractResult<()> {
    let stored_admin = load_admin(&deps.storage)?;
    if sender != &stored_admin {
        Err(StdError::generic_err(
            ContractError::NotAdmin {}.to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Check whether given player provided max amount of coins, that can potentially be lost in the game
pub fn ensure_has_coins_for_game(env: &Env, base_bet: &Coin) -> ContractResult<()> {
    // should be at least 10 x base_bet
    if !has_coins(&env.message.sent_funds, &locked_per_player(base_bet)) {
        Err(StdError::generic_err(
            ContractError::NotEnoughTokensForTheGame {}.to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Ensure base bet is greater then zero
pub fn ensure_correct_base_bet(base_bet: &Coin) -> ContractResult<()> {
    // should be ge 0
    if base_bet.amount.u128() == 0 {
        Err(StdError::generic_err(
            ContractError::BaseBetCanNotBeZero {}.to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Ensure given player joined DAO
pub fn ensure_is_dao_member<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    player: &HumanAddr,
) -> ContractResult<()> {
    // check whether nft supports base bet
    let player_raw = deps.api.canonical_address(player)?;

    load_joiner(&deps.storage, &player_raw)
        .map_err(|_| StdError::generic_err(ContractError::DidNotJoinDao {}.to_string()))
}

/// Ensure given player is not a DAO member
pub fn ensure_is_not_a_dao_member<S: Storage>(
    storage: &S,
    player_raw: &CanonicalAddr,
) -> ContractResult<()> {
    if load_joiner(storage, player_raw).is_ok() {
        Err(StdError::generic_err(
            ContractError::AlreadyJoinedDao {}.to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Ensure provided NFT is in a set of NFTs
pub fn ensure_can_access_nft(player_tokens: Vec<String>, token_id: &str) -> ContractResult<()> {
    if player_tokens
        .iter()
        .any(|player_token| *player_token == *token_id)
    {
        Ok(())
    } else {
        Err(StdError::generic_err(
            ContractError::PlayerCannotAccessProvidedNft {}.to_string(),
        ))
    }
}

/// Ensure given nft can be used in a game
pub fn ensure_can_use_nft_in_a_game<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    token_id: String,
    base_bet: &Coin,
) -> ContractResult<()> {
    let nft_info = query_nft_info_by_id(deps, token_id)?;
    if let NftQueryAnswer::NftInfo { extension, .. } = nft_info {
        if let Some(extension) = extension {
            extension.ensure_enough_xp_for_the_base_bet(base_bet)
        } else {
            Err(StdError::generic_err("NFT extension is not set"))
        }
    } else {
        Err(StdError::generic_err(
            "unable to get NftInfo from nft contract",
        ))
    }
}

/// Get `MintDiceNft` handle message from the parameters provided
fn mint_dice_nft_handle_msg(
    mint_to: &HumanAddr,
    private_metadata: Option<Metadata>,
) -> NftHandleMsg {
    NftHandleMsg::MintDiceNft {
        owner: mint_to.to_owned(),
        private_metadata,
    }
}

/// Get set_nft_metadata_msg from the parameters provided
fn get_set_nft_metadata_msg<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    game_details: &GameDetails,
    winner: Player,
) -> StdResult<CosmosMsg> {
    let token_id = match winner {
        Player::Host => game_details.game.host_player_nft_id.clone(),
        Player::Joined => game_details.game.joined_player_nft_id.clone(),
    };

    let winner_nft_metadata = query_nft_info_by_id(deps, token_id.clone())?;

    let new_ext = if let NftQueryAnswer::NftInfo { extension, .. } = winner_nft_metadata {
        if let Some(mut ext) = extension {
            ext.xp += 5;
            ext
        } else {
            return Err(StdError::generic_err("unable to set metadata with uri"));
        }
    } else {
        return Err(StdError::generic_err(
            "unable to get metadata from nft contract",
        ));
    };

    let set_metadata_msg = NftHandleMsg::SetMetadata {
        token_id,
        public_metadata: Some(Metadata {
            token_uri: None,
            extension: Some(new_ext),
        }),
        private_metadata: None,
        padding: None,
    };

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: nft_address(&deps.storage)?,
        callback_code_hash: nft_code_hash(&deps.storage)?,
        msg: to_binary(&set_metadata_msg)?,
        send: vec![],
    }))
}
