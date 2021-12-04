use crate::error::{ContractError, ContractResult};
use crate::game::{locked_per_player, GameDetails, GameStatus};
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{games, games_mut, last_game_index, PREFIX_LAST_GAME_INDEX};
use cosmwasm_std::{
    coin, has_coins, log, to_binary, Api, BankMsg, Binary, BlockInfo, CanonicalAddr, Coin,
    CosmosMsg, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult,
    Order, Querier, QueryResult, ReadonlyStorage, StdError, StdResult, Storage, WasmMsg,
    KV,
};
use std::convert::TryInto;

pub type GameId = u64;

pub type Secret = u64;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
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

pub fn create_new_game_room<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    nft_id: String,
    base_bet: Coin,
    secret: Secret,
) -> ContractResult<HandleResponse> {
    // check whether nft supports base bet

    ensure_has_coins_for_game(&env, &base_bet)?;

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
    }
}

// query game by it's id
fn query_game<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    game_id: GameId,
) -> StdResult<GameDetails> {
    games(&deps.storage).load(&game_id.to_be_bytes())
}

// returns last game id
fn query_last_game_id<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<u64> {
    last_game_index(&deps.storage).load()
}

// returns a vector of [u8] game keys and their details
fn query_games_by_status<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    status: GameStatus,
) -> StdResult<Vec<(Vec<u8>, GameDetails)>> {
    games(&deps.storage)
        .range(None, None, Order::Ascending)
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
        Err(ContractError::NotEnoughTokensForTheGame {})
    } else {
        Ok(())
    }
}
