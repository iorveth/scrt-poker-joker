use cosmwasm_std::StdError;
use thiserror::Error;

// pub type ContractResult<T> = Result<T, ContractError>;
pub type ContractResult<T> = Result<T, StdError>;

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
    #[error("GivenAccountCannotMakeARoll")]
    GivenAccountCannotMakeARoll {},
    #[error("AlreadyJoined")]
    AlreadyJoined {},
    #[error("QueryPlayerNotValid")]
    QueryPlayerNotValid {},
    #[error("NotEnoughXpForTheBaseBet")]
    NotEnoughXpForTheBaseBet {},
    #[error("GameIsNotFinsihed")]
    GameIsNotFinsihed {},
    #[error("NotAPlayer")]
    NotAPlayer {},
    #[error("BaseBetCanNotBeZero")]
    BaseBetCanNotBeZero {},
}
