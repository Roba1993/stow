pub type Result<O> = std::result::Result<O, StowError>;

#[derive(thiserror::Error, Debug)]
pub enum StowError {
    #[error("IO operation failed")]
    Disconnect(#[from] std::io::Error),

    #[error("Google cloud error")]
    GoogleCloudError(#[from] google_cloud::error::Error),

    #[error("Environment variable missing")]
    EnvironmentVariable(#[from] std::env::VarError),

    #[error("Create Bucket on S3 error")]
    RusotoCreateBucketError(#[from] rusoto_core::RusotoError<rusoto_s3::CreateBucketError>),

    #[error("Region is invalid")]
    RusotoParseRegionError(#[from] rusoto_signature::region::ParseRegionError),

    #[error("Rusoto TLS error")]
    RusotoTlsError(#[from] rusoto_core::request::TlsError),

    #[error("The Item name need a item/file type")]
    ItemTypMissing,

    #[error("The Container cloud not be created")]
    ContainerCreationError,

    #[error("Unknown stow error")]
    Unknown,
}
