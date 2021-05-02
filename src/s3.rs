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

    fn create_client(&self) -> Result<rusoto_s3::S3Client> {
        Ok(rusoto_s3::S3Client::new_with(
            rusoto_core::request::HttpClient::new()?,
            self.credentials.clone(),
            self.region.clone(),
        ))
    }
}

#[async_trait::async_trait]
impl Adapter for S3 {
    async fn containers(&mut self) -> Result<Vec<String>> {
        let client = self.create_client()?;

        let res = rusoto_s3::S3::list_buckets(&client).await?;
        let buckets = res.buckets.ok_or(StowError::ListContainerError)?;

        Ok(buckets
            .into_iter()
            .map(|b| b.name.unwrap_or_default())
            .filter(|s| !s.is_empty())
            .collect())
    }

    async fn create_container(&mut self, container: &str) -> Result<()> {
        let client = self.create_client()?;

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
        let client = self.create_client()?;

        let req = rusoto_s3::DeleteBucketRequest {
            bucket: container.to_string(),
            ..Default::default()
        };

        rusoto_s3::S3::delete_bucket(&client, req).await?;
        Ok(())
    }

    async fn items(&mut self, container: &str) -> Result<Vec<String>> {
        let client = self.create_client()?;

        // inital request
        let mut req = rusoto_s3::ListObjectsV2Request {
            bucket: container.to_string(),
            ..Default::default()
        };

        // response
        let mut res = rusoto_s3::S3::list_objects_v2(&client, req).await?;

        // format to string keys
        let mut items = vec![];
        if let Some(l) = res.contents {
            l.iter()
                .filter_map(|o| o.key.clone())
                .for_each(|o| items.push(o));
        }

        // repeat request as long continuation tokens are available
        while let Some(ct) = res.continuation_token {
            req = rusoto_s3::ListObjectsV2Request {
                bucket: container.to_string(),
                continuation_token: Some(ct),
                ..Default::default()
            };

            res = rusoto_s3::S3::list_objects_v2(&client, req).await?;
            if let Some(l) = res.contents {
                l.iter()
                    .filter_map(|o| o.key.clone())
                    .for_each(|o| items.push(o));
            }
        }

        Ok(items)
    }

    async fn create_item(
        &mut self,
        container: &str,
        item: &str,
        mut reader: impl tokio::io::AsyncRead + Unpin + Send + Sync + 'static,
    ) -> Result<()> {
        use tokio::io::AsyncReadExt;

        let client = self.create_client()?;

        // read the full file into memory to have the content-length
        // todo: this needs to be improved
        let mut data = vec![];
        reader.read_to_end(&mut data).await?;

        // create rusoto byte stream
        let size = data.len() as i64;
        let stream = rusoto_s3::StreamingBody::from(data);

        let req = rusoto_s3::PutObjectRequest {
            bucket: container.to_string(),
            body: Some(stream),
            key: item.to_string(),
            content_length: Some(size),
            ..Default::default()
        };

        rusoto_s3::S3::put_object(&client, req).await?;
        Ok(())
    }

    async fn read_item(
        &mut self,
        container: &str,
        item: &str,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Unpin + Send + Sync>> {
        let client = self.create_client()?;

        let req = rusoto_s3::GetObjectRequest {
            bucket: container.to_string(),
            key: item.to_string(),
            ..Default::default()
        };

        let res = rusoto_s3::S3::get_object(&client, req).await?;
        let res = res.body.ok_or(StowError::EmptyItemError)?;

        Ok(Box::new(res.into_async_read()))
    }

    async fn remove_item(&mut self, container: &str, item: &str) -> Result<()> {
        let client = self.create_client()?;

        let req = rusoto_s3::DeleteObjectRequest {
            bucket: container.to_string(),
            key: item.to_string(),
            ..Default::default()
        };

        rusoto_s3::S3::delete_object(&client, req).await?;
        Ok(())
    }
}
