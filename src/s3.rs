use crate::*;

#[derive(Clone)]
pub struct S3 {
    region: rusoto_core::region::Region,
    credentials: rusoto_credential::StaticProvider,
}

impl S3 {
    pub async fn new(region: &str, access_key: &str, secret_key: &str) -> Result<Self> {
        Ok(S3 {
            region: region.parse()?,
            credentials: rusoto_credential::StaticProvider::new_minimal(
                access_key.into(),
                secret_key.into(),
            ),
        })
    }
}

#[async_trait::async_trait]
impl Adapter for S3 {
    async fn containers(&mut self) -> Result<Vec<String>> {
        todo!();
    }

    async fn create_container(&mut self, container: &str) -> Result<()> {
        let client = rusoto_s3::S3Client::new_with(
            rusoto_core::request::HttpClient::new()?,
            self.credentials.clone(),
            self.region.clone(),
        );

        let bucket_config = rusoto_s3::CreateBucketConfiguration {
            location_constraint: Some(self.region.name().to_string()),
        };

        let req = rusoto_s3::CreateBucketRequest {
            bucket: container.to_string(),
            create_bucket_configuration: Some(bucket_config),
            ..Default::default()
        };

        let res = rusoto_s3::S3::create_bucket(&client, req).await;
        // check if the error is because we own the bucket
        if let Err(rusoto_core::RusotoError::Service(
            rusoto_s3::CreateBucketError::BucketAlreadyOwnedByYou(_),
        )) = &res
        {
            // we already own the bucket - so no error
            return Ok(());
        }

        // unwrap the error if we have an error
        if res?.location.is_none() {
            return Err(StowError::ContainerCreationError);
        }
        Ok(())
    }

    async fn remove_container(&mut self, container: &str) -> Result<()> {
        todo!();
    }

    async fn items(&mut self, container: &str) -> Result<Vec<String>> {
        todo!();
    }

    async fn create_item<'a>(
        &mut self,
        container: &str,
        item: &str,
        mut reader: impl 'a + tokio::io::AsyncRead + Unpin + Send,
    ) -> Result<()> {
        todo!();
    }

    async fn read_item(
        &mut self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send + Sync>> {
        todo!();
    }

    async fn remove_item(&mut self, container: &str, item: &str) -> Result<()> {
        todo!();
    }
}
