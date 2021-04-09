pub type Result<O> = std::result::Result<O, StowError>;

#[derive(thiserror::Error, Debug)]
pub enum StowError {
    #[error("IO operation failed")]
    Disconnect(#[from] std::io::Error),

    #[error("The Item name need a item/file type")]
    ItemTypMissing,

    #[error("Unknown stow error")]
    Unknown,
}
