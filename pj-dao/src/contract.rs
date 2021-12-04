use crate::error::{ContractError, ContractResult};
use crate::game::{locked_per_player, GameDetails, GameStatus};
use crate::msg::{HandleMsg, InitMsg, NftInitMsg, PostInitCallback, QueryMsg};
use crate::state::{
    load_game, load_last_game_index, nft_address, save_game, save_last_game_index,
    save_nft_address, PREFIX_LAST_GAME_INDEX, PREFIX_NFT_CONTRACT,
};
use cosmwasm_std::{
    coin, has_coins, log, to_binary, Api, BankMsg, Binary, BlockInfo, CanonicalAddr, Coin,
    CosmosMsg, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    Order, Querier, QueryResult, ReadonlyStorage, StdError, StdResult, Storage, WasmMsg, KV,
};
use std::convert::TryInto;

pub type GameId = u64;

pub type Secret = u64;

pub const INIT_INDEX: GameId = 0;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    save_last_game_index(&mut deps.storage, &INIT_INDEX)?;
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
        HandleMsg::CreateNFTContract { code_id } => create_nft_contract(deps, env, code_id),
        HandleMsg::StoreNFTContract {} => store_nft_contract_addr(deps, env),
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
        HandleMsg::Roll { game_id } => unimplemented!(),
        HandleMsg::ReRoll { game_id, dices } => unimplemented!(),
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
    _deps: &mut Extern<S, A, Q>,
    env: Env,
    code_id: u64,
) -> ContractResult<HandleResponse> {
    let store_addr_msg = HandleMsg::StoreNFTContract {};
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
        callback_code_hash: env.contract_code_hash,
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

    let game = GameDetails::new(host_player_address, nft_id, base_bet);

    // save newly initialized game
    save_game(&mut deps.storage, game_id, &game)?;

    // increment game index
    save_last_game_index(&mut deps.storage, &(game_id + 1))?;

    Ok(HandleResponse::default())
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
    let game = query_game(deps, game_id)?;

    // ensure enough coins provided
    ensure_has_coins_for_game(&env, &game.base_bet)?;

    // ensure game status is set to pending
    game.status.ensure_is_pending()?;

    Ok(HandleResponse::default())
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
) -> StdResult<GameDetails> {
    load_game(&deps.storage, game_id)
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
) -> StdResult<Vec<(GameId, GameDetails)>> {
    (INIT_INDEX..=load_last_game_index(&deps.storage)?)
        .into_iter()
        .map(|i| match load_game(&deps.storage, i) {
            Ok(game) => Ok((i, game)),
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
