use crate::*;

#[derive(Clone)]
pub struct Gcs {
    client: google_cloud::storage::Client,
}

impl Gcs {
    pub async fn new(project_name: &str, path: &str) -> Result<Self> {
        std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", path);

        Ok(Self {
            client: google_cloud::storage::Client::new(project_name).await?,
        })
    }
}

#[async_trait::async_trait(?Send)]
impl Adapter for Gcs {
    async fn containers(&mut self) -> Result<Vec<String>> {
        Ok(self
            .client
            .buckets()
            .await?
            .into_iter()
            .map(|b| b.name().to_string())
            .collect())
    }

    async fn create_container(&mut self, container: &str) -> Result<()> {
        let container = util::streamline(container);

        // only create bucket if not avialble
        if self.client.bucket(&container).await.is_err() {
            self.client
                .create_bucket(&util::streamline(&container))
                .await?;
        };

        Ok(())
    }

    async fn remove_container(&mut self, container: &str) -> Result<()> {
        let bucket = self.client.bucket(&util::streamline(container)).await?;
        bucket.delete().await?;
        Ok(())
    }

    async fn items(&mut self, container: &str) -> Result<Vec<String>> {
        let mut bucket = self.client.bucket(&util::streamline(container)).await?;
        Ok(bucket
            .objects()
            .await?
            .into_iter()
            .map(|o| o.name().to_string())
            .collect())
    }

    async fn create_item(
        &mut self,
        container: &str,
        item: &str,
        reader: &mut (impl tokio::io::AsyncRead + Unpin),
    ) -> Result<()> {
        use tokio::io::AsyncReadExt;

        let container = util::streamline(container);
        let item = util::streamline_item(item)?;

        let mut bucket = self.client.bucket(&container).await?;

        let mut data = vec![];
        reader.read_to_end(&mut data).await?;

        bucket
            .create_object(&item, data, "application/octet-stream")
            .await?;

        Ok(())
    }

    async fn read_item(
        &mut self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin>> {
        let container = util::streamline(container);
        let item = util::streamline_item(item)?;

        let mut bucket = self.client.bucket(&container).await?;
        let mut object = bucket.object(&item).await?;

        Ok(Box::new(object.reader().await?))
    }

    async fn remove_item(&mut self, container: &str, item: &str) -> Result<()> {
        let container = util::streamline(container);
        let item = util::streamline_item(item)?;

        let mut bucket = self.client.bucket(&container).await?;
        let object = bucket.object(&item).await?;
        object.delete().await?;
        Ok(())
    }
}
