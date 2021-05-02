use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_s3() -> stow::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        if !e.not_found() {
            let e: Result<(), dotenv::Error> = Err(e);
            e.unwrap();
        }
    }

    let access_key = std::env::var("STOW_AWS_ACCESS_KEY")?;
    let secret_key = std::env::var("STOW_AWS_SECRET_KEY")?;
    let container_1 = std::env::var("STOW_TEST_CONTAINER_1")?;
    let container_2 = std::env::var("STOW_TEST_CONTAINER_2")?;

    // create a new environment if not avilable
    let mut aws3 = stow::Location::new_s3("eu-central-1", &access_key, &secret_key).await?;

    // create new containers if not avilable
    aws3.create_container(&container_1).await?;
    aws3.create_container(&container_2).await?;

    assert!(aws3
        .containers()
        .await?
        .contains(&String::from(&container_1)));
    assert!(aws3
        .containers()
        .await?
        .contains(&String::from(&container_2)));

    // create two test.txt file
    aws3.create_item(&container_1, "test.txt", reader("Hello World 1").await?)
        .await?;
    aws3.create_item(&container_2, "test.txt", reader("Hello World 2").await?)
        .await?;

    assert!(aws3
        .items(&container_2)
        .await?
        .contains(&String::from("test.txt")));

    // rewrite the test.txt file
    aws3.create_item(&container_1, "test.txt", reader("Hello World 1 New").await?)
        .await?;

    // read the test.txt file
    let mut buf = vec![];
    aws3.read_item(&container_1, "test.txt")
        .await?
        .read_to_end(&mut buf)
        .await?;
    assert_eq!(&b"Hello World 1 New"[0..], &buf);

    // remove the item.txt in container 2
    aws3.remove_item(&container_2, "test.txt").await?;
    assert!(aws3.read_item(&container_2, "test.txt").await.is_err());
    // remove the container 2
    aws3.remove_container(&container_2).await?;

    // remove the item.txt in container 1
    aws3.remove_item(&container_1, "test.txt").await?;
    assert!(aws3.read_item(&container_1, "test.txt").await.is_err());
    aws3.remove_container(&container_1).await?;

    Ok(())
}

async fn reader(data: &str) -> stow::Result<tokio::io::DuplexStream> {
    let (mut send, recv) = tokio::io::duplex(data.len());
    send.write_all(data.as_bytes()).await?;
    send.shutdown().await?;
    Ok(recv)
}
