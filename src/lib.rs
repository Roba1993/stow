mod error;
mod local;

pub use error::*;
pub use local::*;

#[async_trait::async_trait(?Send)]
pub trait Adapter: Clone {
    async fn containers(&self) -> Result<Vec<String>>;
    async fn create_container(&self, container: &str) -> Result<()>;
    async fn remove_container(&self, container: &str) -> Result<()>;

    async fn items(&self, container: &str) -> Result<Vec<String>>;
    async fn create_item(
        &self,
        container: &str,
        item: &str,
        reader: &mut (impl tokio::io::AsyncRead + Unpin),
    ) -> Result<()>;
    async fn read_item(
        &self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin>>;
    async fn remove_item(&self, container: &str, item: &str) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum Location {
    Local(LocalLocation),
}

impl Location {
    /// Create a new local Location with the given path
    pub async fn new_local(path: &str) -> Result<Self> {
        Ok(Location::Local(LocalLocation::new(path).await?))
    }

    pub async fn containers(&self) -> Result<Vec<String>> {
        match self {
            Location::Local(l) => l.containers().await,
        }
    }

    pub async fn create_container(&self, container: &str) -> Result<()> {
        match self {
            Location::Local(l) => l.create_container(container).await,
        }
    }

    pub async fn remove_container(&self, container: &str) -> Result<()> {
        match self {
            Location::Local(l) => l.remove_container(container).await,
        }
    }

    pub async fn items(&self, container: &str) -> Result<Vec<String>> {
        match self {
            Location::Local(l) => l.items(container).await,
        }
    }

    pub async fn create_item(
        &self,
        container: &str,
        item: &str,
        reader: &mut (impl tokio::io::AsyncRead + Unpin),
    ) -> Result<()> {
        match self {
            Location::Local(l) => l.create_item(container, item, reader).await,
        }
    }

    pub async fn read_item(
        &self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin>> {
        match self {
            Location::Local(l) => l.read_item(container, item).await,
        }
    }

    pub async fn remove_item(&self, container: &str, item: &str) -> Result<()> {
        match self {
            Location::Local(l) => l.remove_item(container, item).await,
        }
    }
}

mod util {
    use super::*;

    pub fn streamline(input: &str) -> String {
        // reformat the name
        let reg = regex::Regex::new("[^a-z0-9_]").unwrap();
        let mut res = reg.replace_all(input.to_lowercase().trim(), "").to_string();
        if res.starts_with('_') {
            res.remove(0);
        }
        res
    }

    pub fn streamline_item(input: &str) -> Result<String> {
        let pos = input.find('.').ok_or(StowError::ItemTypMissing)?;

        // check if a file is defined and if it has an ending
        let (base, typ) = input.split_at(pos);

        // reformat the name
        let reg = regex::Regex::new("[^a-z0-9_]").unwrap();
        let mut res = reg.replace_all(base.to_lowercase().trim(), "").to_string();
        if res.starts_with('_') {
            res.remove(0);
        }

        // add the file ending if it is a item
        let mut out = String::from(&res);
        out.push_str(&typ.to_lowercase());

        Ok(out)
    }
}
