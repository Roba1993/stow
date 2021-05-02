use crate::*;

#[derive(Debug, Clone)]
pub struct LocalLocation {
    path: String,
}

impl LocalLocation {
    pub async fn new(path: &str) -> Result<Self> {
        let this = Self {
            path: util::streamline(path),
        };
        tokio::fs::create_dir_all(&this.path).await?;
        Ok(this)
    }
}

#[async_trait::async_trait]
impl Adapter for LocalLocation {
    async fn containers(&mut self) -> Result<Vec<String>> {
        let mut res = tokio::fs::read_dir(&self.path).await?;
        let mut containers = vec![];

        while let Some(con) = res.next_entry().await? {
            if let Some(name) = con.file_name().to_str() {
                containers.push(name.to_string());
            }
        }

        Ok(containers)
    }

    async fn create_container(&mut self, container: &str) -> Result<()> {
        let mut path = self.path.clone();
        path.push('/');
        path.push_str(container);

        if let Err(e) = tokio::fs::create_dir(path).await {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e.into());
            }
        }

        Ok(())
    }

    async fn remove_container(&mut self, container: &str) -> Result<()> {
        let mut path = self.path.clone();
        path.push('/');
        path.push_str(&util::streamline(container));

        tokio::fs::remove_dir_all(path).await?;
        Ok(())
    }

    async fn items(&mut self, container: &str) -> Result<Vec<String>> {
        let container = util::streamline(container);
        let mut path = String::from(&self.path);
        path.push('/');
        path.push_str(&container);

        let mut res = tokio::fs::read_dir(&path).await?;
        let mut containers = vec![];

        while let Some(con) = res.next_entry().await? {
            if let Some(name) = con.file_name().to_str() {
                containers.push(name.to_string());
            }
        }

        Ok(containers)
    }

    async fn create_item(
        &mut self,
        container: &str,
        item: &str,
        mut reader: impl tokio::io::AsyncRead + Unpin + Send + 'static,
    ) -> Result<()> {
        let item = util::streamline_item(item)?;

        let mut path = String::from(&self.path);
        path.push('/');
        path.push_str(&container);
        path.push('/');
        path.push_str(&item);

        let mut file = tokio::fs::File::create(path).await?;
        tokio::io::copy(&mut reader, &mut file).await?;

        Ok(())
    }

    async fn read_item(
        &mut self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send + Sync>> {
        let container = util::streamline(container);
        let item = util::streamline_item(item)?;

        let mut path = String::from(&self.path);
        path.push('/');
        path.push_str(&container);
        path.push('/');
        path.push_str(&item);

        let file = tokio::fs::File::open(path).await?;
        Ok(Box::new(file))
    }

    async fn remove_item(&mut self, container: &str, item: &str) -> Result<()> {
        let container = util::streamline(container);
        let item = util::streamline_item(item)?;

        let mut path = String::from(&self.path);
        path.push('/');
        path.push_str(&container);
        path.push('/');
        path.push_str(&item);

        tokio::fs::remove_file(path).await?;
        Ok(())
    }
}
