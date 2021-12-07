use crate::error::{ContractError, ContractResult};
use crate::game::{locked_per_player, Game, GameDetails, GameStatus, NUM_OF_DICES};
use crate::msg::{
    HandleMsg, InitMsg, JoinNftDetails, NftHandleMsg, NftInitMsg, NftQueryAnswer, NftQueryMsg,
    PostInitCallback, QueryMsg, ViewerInfo,
};
use crate::state::{
    load_game, load_joiner, load_last_game_index, nft_address, nft_code_hash, nft_code_id,
    save_game, save_joiner, save_last_game_index, save_nft_address, save_nft_code_hash,
    save_nft_code_id, PREFIX_LAST_GAME_INDEX, PREFIX_NFT_CONTRACT,
};
use cosmwasm_std::{
    coin, has_coins, log, to_binary, Api, BankMsg, Binary, BlockInfo, CanonicalAddr, Coin,
    CosmosMsg, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    LogAttribute, Order, Querier, QueryRequest, QueryResult, ReadonlyStorage, StdError, StdResult,
    Storage, WasmMsg, WasmQuery, KV,
};
use std::convert::TryInto;
use std::os::unix::prelude::OsStrExt;

pub type GameId = u64;

pub type Secret = u64;

pub const INIT_INDEX: GameId = 0;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    save_last_game_index(&mut deps.storage, &INIT_INDEX)?;
    save_nft_code_hash(&mut deps.storage, msg.nft_code_hash)?;
    save_nft_code_id(&mut deps.storage, msg.nft_code_id)?;

    Ok(InitResponse::default())
}

// pub fn winner_winner_chicken_dinner(
//     contract_address: HumanAddr,
//     player: HumanAddr,
//     amount: Uint128,
// ) -> HandleResponse {
//     HandleResponse {
//         messages: vec![CosmosMsg::Bank(BankMsg::Send {
//             from_address: contract_address,
//             to_address: player,
//             amount: vec![Coin {
//                 denom: "uscrt".to_string(),
//                 amount,
//             }],
//         })],
//         log: vec![],
//         data: None,
//     }
// }

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
    }
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
            // check this belongs to the user
            // TODO query
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
            save_joiner(&mut deps.storage, &sender_raw, nft.viewing_key)?;
        } else {
            // we will mint a new nft for the owner
            let viewing_key = String::from_utf8_lossy(
                &[
                    env.message.sender.0.as_bytes(),
                    &env.block.time.to_be_bytes(),
                ]
                .concat(),
            )
            .into_owned();

            let mint_dice_msg = NftHandleMsg::MintDiceNft {
                owner: Some(env.message.sender.clone()),
                key: viewing_key.clone(),
                private_metadata: None,
            };

            // save the new joiner's viewing key
            save_joiner(&mut deps.storage, &sender_raw, viewing_key)?;

            let contract_addr = nft_address(&deps.storage)?;
            let callback_code_hash = nft_code_hash(&deps.storage)?;
            let mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr,
                callback_code_hash,
                msg: to_binary(&mint_dice_msg)?,
                send: vec![],
            });
            response_msg.push(mint_msg);
        }
        Ok(HandleResponse {
            messages: response_msg,
            log: vec![log("joined", env.message.sender)],
            data: None,
        })
    } else {
        Err(StdError::generic_err(
            ContractError::AlreadyJoined {}.to_string(),
        ))
    }
}

pub fn store_nft_contract_addr<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> ContractResult<HandleResponse> {
    save_nft_address(&mut deps.storage, &env.message.sender)?;
    Ok(HandleResponse::default())
}

pub fn create_nft_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> ContractResult<HandleResponse> {
    let code_id = nft_code_id(&deps.storage)?;
    let callback_code_hash = nft_code_hash(&deps.storage)?;
    let store_addr_msg = HandleMsg::StoreNftContract {};
    let post_init_callback = PostInitCallback {
        msg: to_binary(&store_addr_msg)?,
        contract_address: env.contract.address.clone(),
        code_hash: env.contract_code_hash.clone(),
        send: vec![],
    };
    let nft_instantiate_msg = NftInitMsg {
        name: "PokerJokerNFT".into(),
        symbol: "PJX".into(),
        admin: Some(env.contract.address),
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
        log: vec![],
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

    let host_player_address = deps.api.canonical_address(&env.message.sender)?;

    // ensure enough coins provided
    ensure_has_coins_for_game(&env, &base_bet)?;

    let game_id = load_last_game_index(&deps.storage)?;

    // create new game with provided host player secret
    let game = Game::new(host_player_address, nft_id, base_bet);
    let game_details = GameDetails::new(game, secret.to_be_bytes());

    // save newly initialized game
    save_game(&mut deps.storage, game_id, &game_details)?;

    // increment game index
    save_last_game_index(&mut deps.storage, &(game_id + 1))?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
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
    let joined_player_address = deps.api.canonical_address(&env.message.sender)?;

    // check whether nft supports base_bet

    // check whether dao member

    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    // ensure enough coins provided
    ensure_has_coins_for_game(&env, &game_details.game.base_bet)?;

    // ensure game status is set to pending
    game_details.ensure_is_pending()?;

    // join the game
    game_details.join(joined_player_address, nft_id, secret.to_be_bytes());

    // save updated game state
    save_game(&mut deps.storage, game_id, &game_details)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: None,
    })
}

pub fn roll<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    game_id: GameId,
) -> ContractResult<HandleResponse> {
    let joined_player_address = deps.api.canonical_address(&env.message.sender)?;

    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    // ensure game status is set to started
    game_details.ensure_is_started()?;

    // Ensure given account can now make a roll in a game
    game_details.ensure_can_roll(joined_player_address)?;

    game_details.roll(game_id);

    // save updated game state
    save_game(&mut deps.storage, game_id, &game_details)?;

    // check whether player can roll
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: None,
    })
}

pub fn reroll<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    game_id: GameId,
    dices: [bool; NUM_OF_DICES],
) -> ContractResult<HandleResponse> {
    let joined_player_address = deps.api.canonical_address(&env.message.sender)?;

    // ensure game exists
    let mut game_details = load_game(&deps.storage, game_id)?;

    // ensure game status is set to started
    game_details.ensure_is_started()?;

    // Ensure given account can make a roll in a game
    game_details.ensure_can_roll(joined_player_address)?;

    game_details.reroll(dices);

    // Game storage removal
    // if game_details.is_finished() {
    //     // remove game after completion
    //     remove_game(&mut deps.storage, game_id, &game_details)?;
    // }

    // check whether player can roll
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
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
    }
}

// query game by it's id
fn query_game<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    game_id: GameId,
) -> StdResult<Game> {
    load_game(&deps.storage, game_id).map(|game_details| Game::from(game_details))
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
