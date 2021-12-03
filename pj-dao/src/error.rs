use cosmwasm_std::StdError;
use thiserror::Error;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("NotEnoughTokensForTheGame")]
    NotEnoughTokensForTheGame {},
    #[error("GameNotInPendingStatus")]
    GameNotInPendingStatus {},
    #[error("GameNotInStartedStatus")]
    GameNotInStartedStatus {},
    #[error("GameNotInRerollStatus")]
    GameNotInRerollStatus {},
}
