mod error;
mod gcs;
mod local;
mod s3;

pub use error::*;
pub use gcs::*;
pub use local::*;
pub use s3::*;

#[async_trait::async_trait]
pub trait Adapter: Clone {
    async fn containers(&mut self) -> Result<Vec<String>>;
    async fn create_container(&mut self, container: &str) -> Result<()>;
    async fn remove_container(&mut self, container: &str) -> Result<()>;

    async fn items(&mut self, container: &str) -> Result<Vec<String>>;
    async fn create_item<'a>(
        &mut self,
        container: &str,
        item: &str,
        reader: (impl 'a + tokio::io::AsyncRead + Unpin + Send),
    ) -> Result<()>;
    async fn read_item(
        &mut self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send + Sync>>;
    async fn remove_item(&mut self, container: &str, item: &str) -> Result<()>;
}

#[derive(Clone)]
pub enum Location {
    Local(LocalLocation),
    Gcs(Gcs),
    S3(S3),
}

impl Location {
    /// Create a new local location with the given path
    pub async fn new_local(path: &str) -> Result<Self> {
        Ok(Location::Local(LocalLocation::new(path).await?))
    }

    /// Create a new gcs location with the given project
    /// The google service account details need to be stored in the json file.
    /// The path to the json file, need to be set as path
    pub async fn new_gcs(project: &str, path: &str) -> Result<Self> {
        Ok(Location::Gcs(Gcs::new(project, path).await?))
    }

    /// Create a new S3 location with the given region and credentials
    pub async fn new_s3(region: &str, access_key: &str, secret_key: &str) -> Result<Self> {
        Ok(Location::S3(S3::new(region, access_key, secret_key).await?))
    }

    pub async fn containers(&mut self) -> Result<Vec<String>> {
        match self {
            Location::Local(l) => l.containers().await,
            Location::Gcs(l) => l.containers().await,
            Location::S3(l) => l.containers().await,
        }
    }

    pub async fn create_container(&mut self, container: &str) -> Result<()> {
        let container = util::streamline(&container);

        match self {
            Location::Local(l) => l.create_container(&container).await,
            Location::Gcs(l) => l.create_container(&container).await,
            Location::S3(l) => l.create_container(&container).await,
        }
    }

    pub async fn remove_container(&mut self, container: &str) -> Result<()> {
        match self {
            Location::Local(l) => l.remove_container(container).await,
            Location::Gcs(l) => l.remove_container(container).await,
            Location::S3(l) => l.remove_container(container).await,
        }
    }

    pub async fn items(&mut self, container: &str) -> Result<Vec<String>> {
        match self {
            Location::Local(l) => l.items(container).await,
            Location::Gcs(l) => l.items(container).await,
            Location::S3(l) => l.items(container).await,
        }
    }

    pub async fn create_item(
        &mut self,
        container: &str,
        item: &str,
        reader: (impl tokio::io::AsyncRead + Unpin + Send),
    ) -> Result<()> {
        match self {
            Location::Local(l) => l.create_item(container, item, reader).await,
            Location::Gcs(l) => l.create_item(container, item, reader).await,
            Location::S3(l) => l.create_item(container, item, reader).await,
        }
    }

    pub async fn read_item(
        &mut self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send + Sync>> {
        match self {
            Location::Local(l) => l.read_item(container, item).await,
            Location::Gcs(l) => l.read_item(container, item).await,
            Location::S3(l) => l.read_item(container, item).await,
        }
    }

    pub async fn remove_item(&mut self, container: &str, item: &str) -> Result<()> {
        match self {
            Location::Local(l) => l.remove_item(container, item).await,
            Location::Gcs(l) => l.remove_item(container, item).await,
            Location::S3(l) => l.remove_item(container, item).await,
        }
    }
}

mod util {
    use super::*;

    pub fn streamline(input: &str) -> String {
        // reformat the name
        let reg = regex::Regex::new("[^a-z0-9\\-]").unwrap();
        let mut res = reg.replace_all(input.to_lowercase().trim(), "").to_string();
        if res.starts_with('-') {
            res.remove(0);
        }
        res
    }

    pub fn streamline_item(input: &str) -> Result<String> {
        let pos = input.find('.').ok_or(StowError::ItemTypMissing)?;

        // check if a file is defined and if it has an ending
        let (base, typ) = input.split_at(pos);

        // reformat the name
        let reg = regex::Regex::new("[^a-z0-9\\-]").unwrap();
        let mut res = reg.replace_all(base.to_lowercase().trim(), "").to_string();
        if res.starts_with('-') {
            res.remove(0);
        }

        // add the file ending if it is a item
        let mut out = String::from(&res);
        out.push_str(&typ.to_lowercase());

        Ok(out)
    }
}
