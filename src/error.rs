use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum RpgError {
    #[error("failed to open given file")]
    Io(#[from] std::io::Error),
    #[error("failed to get a successful response: {0}")]
    Reqwest(#[from] reqwest::Error),
}
