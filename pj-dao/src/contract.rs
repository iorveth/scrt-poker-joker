use crate::game::GameDetails;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{
    json_may_load, json_save, load, may_load, remove, save, PREFIX_ACITVE_GAMES, PREFIX_JOINERS,
    PREFIX_PENDING_GAMES,
};
use cosmwasm_std::{
    log, to_binary, Api, BankMsg, Binary, BlockInfo, CanonicalAddr, Coin, CosmosMsg, Env, Extern,
    HandleResponse, HandleResult, HumanAddr, InitResponse, InitResult, Querier, QueryResult,
    ReadonlyStorage, StdError, StdResult, Storage, WasmMsg,
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateDiceNFTContract { code_id } => {
            instantiate_nft_contract(deps, env, code_id)
        }
        HandleMsg::JoinDao {} => join_dao(deps, env),
        HandleMsg::StartNewGame { nft_id, base_bet } => start_new_game(deps, env, nft_id, base_bet),
        HandleMsg::JoinGame { nft_id, game_id } => join_game(deps, env, nft_id, game_id),
        HandleMsg::Roll { game_id } => unimplemented!(),
        HandleMsg::ReRoll {
            game_id,
            num_of_dice,
        } => unimplemented!(),
        HandleMsg::EndGame { game_id } => unimplemented!(),
    }
}

pub fn instantiate_nft_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    code_id: u64,
) -> HandleResult {
    unimplemented!()
}

pub fn join_dao<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    unimplemented!()
}

pub fn start_new_game<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    nft_id: String,
    base_bet: Coin,
) -> HandleResult {
    unimplemented!()
}

pub fn join_game<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    nft_id: String,
    game_id: u32,
) -> HandleResult {
    unimplemented!()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        // ActiveGameIds {},
        // PendingGameIds {},
        // ActiveGames {},
        // PendingGames {},
        // Game { GameId: u32 },
        QueryMsg::ActiveGames {} => to_binary(&query_active_games(deps)?),
        QueryMsg::PendingGames {} => to_binary(&query_pending_games(deps)?),
        QueryMsg::Game { game_id } => to_binary(&query_game(deps, game_id)?),
    }
}

fn query_active_games<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Vec<GameDetails>> {
    unimplemented!();
}

fn query_pending_games<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Vec<GameDetails>> {
    unimplemented!();
}

fn query_game<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    game_id: u32,
) -> StdResult<Vec<GameDetails>> {
    unimplemented!();
}
