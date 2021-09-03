use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("rate limit")]
    RateLimit(),
    #[error("parse failed")]
    Parse(#[from] serde_json::Error),
    #[error("request failed")]
    Request(#[from] reqwest::Error),
    #[error("bot failed")]
    RequestError(#[from] teloxide::RequestError),
    #[error("unknown data store error")]
    Unknown,
}
