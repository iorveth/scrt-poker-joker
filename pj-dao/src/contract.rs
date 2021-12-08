use crate::error::{ContractError, ContractResult};
use crate::game::{locked_per_player, Game, GameDetails, GameStatus, Player, NUM_OF_DICES};
use crate::msg::{
    HandleMsg, InitMsg, JoinNftDetails, Metadata, MintDiceNft, NftHandleMsg, NftInfo, NftInitMsg,
    NftQueryAnswer, NftQueryMsg, PostInitCallback, QueryMsg, SetMetadata, ViewerInfo,
};
use crate::state::{
    load_admin, load_game, load_joiner, load_last_game_index, nft_address, nft_code_hash,
    nft_code_id, remove_game, save_admin, save_game, save_joiner, save_last_game_index,
    save_nft_address, save_nft_code_hash, save_nft_code_id,
};
use cosmwasm_std::{
    has_coins, log, to_binary, Api, Binary, Coin, CosmosMsg, Env, Extern, HandleResponse,
    HumanAddr, InitResponse, Querier, QueryRequest, StdError, StdResult, Storage, WasmMsg,
    WasmQuery,
};
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
        } => create_new_game_room(deps, env, nft_id, base_bet, secret),
        HandleMsg::JoinGame {
            nft_id,
            game_id,
            secret,
        } => join_game(deps, env, nft_id, game_id, secret),
        HandleMsg::Roll { game_id } => roll(deps, env, game_id),
        HandleMsg::ReRoll { game_id, dices } => reroll(deps, env, game_id, dices),
        HandleMsg::AdminMint(admin_mint_data) => admin_mint(
            deps,
            env,
            admin_mint_data.to,
            admin_mint_data.private_metadata,
        ),
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
    let (msg, viewing_key) = mint_dice_nft_handle_msg(&to, env.block.time, private_metadata);

    save_joiner(
        &mut deps.storage,
        &deps.api.canonical_address(&to)?,
        viewing_key,
    )?;

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
    // TODO: charge user?
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    let is_joined = load_joiner(&deps.storage, &sender_raw)?;

    if is_joined == None {
        let mut response_msg: Vec<CosmosMsg> = vec![];

        // if a nft_id is provided, we check if msg.sender owns the nft
        // in order to check, dao must be provided the viewing key, at least this once
        if let Some(nft) = nft {
            let query = WasmQuery::Smart {
                contract_addr: nft_address(&deps.storage)?,
                callback_code_hash: nft_code_hash(&deps.storage)?,
                msg: to_binary(&NftQueryMsg::OwnerOf {
                    token_id: nft.id,
                    viewer: Some(ViewerInfo {
                        address: env.message.sender.clone(),
                        viewing_key: nft.viewing_key.clone(),
                    }),
                    include_expired: None,
                })?,
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
            save_joiner(&mut deps.storage, &sender_raw, nft.viewing_key)?;
        } else {
            // we will mint a new nft for the owner
            let (msg, viewing_key) =
                mint_dice_nft_handle_msg(&env.message.sender, env.block.time, None);

            // save the new joiner's viewing key
            save_joiner(&mut deps.storage, &sender_raw, viewing_key)?;

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
    } else {
        Err(StdError::generic_err(
            ContractError::AlreadyJoined {}.to_string(),
        ))
    }
}

fn mint_dice_nft_handle_msg(
    mint_to: &HumanAddr,
    block_time: u64,
    private_metadata: Option<Metadata>,
) -> (NftHandleMsg, String) {
    let viewing_key =
        String::from_utf8_lossy(&[mint_to.0.as_bytes(), &block_time.to_be_bytes()].concat())
            .into_owned();

    let mint_dice_msg = Box::new(MintDiceNft {
        owner: mint_to.clone(),
        key: viewing_key.clone(),
        private_metadata,
    });

    (NftHandleMsg::MintDiceNft(mint_dice_msg), viewing_key)
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
) -> ContractResult<HandleResponse> {
    // check whether nft supports base bet

    // check whether dao member

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
) -> ContractResult<HandleResponse> {
    // check whether nft supports base_bet

    // check whether dao member

    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    // ensure enough coins provided
    ensure_has_coins_for_game(&env, &game_details.game.base_bet)?;

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
    let set_metadata_msg = if let Some(winner) = winner {
        let token_id = match winner {
            Player::Host => game_details.game.host_player_nft_id.clone(),
            Player::Joined => game_details.game.joined_player_nft_id.clone(),
        };

        let current_winner_metadata: NftQueryAnswer =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: nft_address(&deps.storage)?,
                /// callback_code_hash is the hex encoded hash of the code. This is used by Secret Network to harden against replaying the contract
                /// It is used to bind the request to a destination contract in a stronger way than just the contract address which can be faked
                callback_code_hash: nft_code_hash(&deps.storage)?,
                /// msg is the json-encoded QueryMsg struct
                msg: to_binary(&NftQueryMsg::NftInfo {
                    token_id: token_id.clone(),
                })?,
            }))?;

        let new_ext = match current_winner_metadata {
            NftQueryAnswer::NftInfo(nft_info) => {
                if let NftInfo {
                    extension: Some(mut ext),
                    ..
                } = *nft_info
                {
                    ext.xp += 5;
                    ext
                } else {
                    return Err(StdError::generic_err("unable to set metadata with uri"));
                }
            }
            _ => {
                return Err(StdError::generic_err(
                    "unable to get metadata from nft contract",
                ));
            }
        };

        let set_metadata_msg = Box::new(SetMetadata {
            token_id,
            public_metadata: Some(Metadata {
                token_uri: None,
                extension: Some(new_ext),
            }),
            private_metadata: None,
            padding: None,
        });

        Some(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: nft_address(&deps.storage)?,
            callback_code_hash: nft_code_hash(&deps.storage)?,
            msg: to_binary(&NftHandleMsg::SetMetadata(set_metadata_msg))?,
            send: vec![],
        }))
    } else {
        None
    };

    let mut messages = game_details.complete_checkout(env.contract.address, winner);

    if let Some(msg) = set_metadata_msg {
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
        QueryMsg::PlayerNfts { player, viewer } => {
            to_binary(&query_player_nfts(deps, player, viewer)?)
        }
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

fn query_player_nfts<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    player: HumanAddr,
    viewer: HumanAddr,
) -> StdResult<Vec<String>> {
    let player_raw = deps.api.canonical_address(&player)?;
    if let Some(viewing_key) = load_joiner(&deps.storage, &player_raw)? {
        let query = NftQueryMsg::Tokens {
            owner: player,
            /// optional address of the querier if different from the owner
            viewer: Some(viewer),
            /// optional viewing key
            viewing_key: Some(viewing_key),
            /// paginate by providing the last token_id received in the previous query
            start_after: None,
            /// optional number of token ids to display
            limit: None,
        };

        let tokens: NftQueryAnswer = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: nft_address(&deps.storage)?,
            /// callback_code_hash is the hex encoded hash of the code. This is used by Secret Network to harden against replaying the contract
            /// It is used to bind the request to a destination contract in a stronger way than just the contract address which can be faked
            callback_code_hash: nft_code_hash(&deps.storage)?,
            /// msg is the json-encoded QueryMsg struct
            msg: to_binary(&query)?,
        }))?;

        match tokens {
            NftQueryAnswer::TokenList { tokens: list } => Ok(list),
            _ => Err(StdError::generic_err(
                ContractError::QueryPlayerNotValid {}.to_string(),
            )),
        }
    } else {
        Err(StdError::generic_err(
            ContractError::QueryPlayerNotValid {}.to_string(),
        ))
    }
}

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

// Check whether given player provided max amount of coins, that can potentially be lost in the game
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
