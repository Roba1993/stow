pub type Result<O> = std::result::Result<O, StowError>;

#[derive(thiserror::Error, Debug)]
pub enum StowError {
    #[error("IO operation failed")]
    Disconnect(#[from] std::io::Error),

    #[error("Google cloud error")]
    GoogleCloudError(#[from] google_cloud::error::Error),

    #[error("Environment variable missing")]
    EnvironmentVariable(#[from] std::env::VarError),

    #[error("The Item name need a item/file type")]
    ItemTypMissing,

    #[error("Unknown stow error")]
    Unknown,
}
