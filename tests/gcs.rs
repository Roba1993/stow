use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_gcs() -> stow::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        if !e.not_found() {
            let e: Result<(), dotenv::Error> = Err(e);
            e.unwrap();
        }
    }

    let project = std::env::var("STOW_TEST_GCP_PROJECT")?;
    let path = std::env::var("STOW_TEST_GCP_ACCESS_PATH")?;
    let container_1 = std::env::var("STOW_TEST_CONTAINER_1")?;
    let container_2 = std::env::var("STOW_TEST_CONTAINER_2")?;

    // create a new environment if not avilable
    let mut gcs = stow::Location::new_gcs(&project, &path).await?;

    // create new containers if not avilable
    gcs.create_container(&container_1).await?;
    gcs.create_container(&container_2).await?;

    assert!(gcs
        .containers()
        .await?
        .contains(&String::from(&container_1)));
    assert!(gcs
        .containers()
        .await?
        .contains(&String::from(&container_2)));

    // create two test.txt file
    gcs.create_item(&container_1, "test.txt", reader("Hello World 1").await?)
        .await?;
    gcs.create_item(&container_2, "test.txt", reader("Hello World 2").await?)
        .await?;

    assert!(gcs
        .items(&container_2)
        .await?
        .contains(&String::from("test.txt")));

    // rewrite the test.txt file
    gcs.create_item(&container_1, "test.txt", reader("Hello World 1 New").await?)
        .await?;

    // read the test.txt file
    let mut buf = vec![];
    gcs.read_item(&container_1, "test.txt")
        .await?
        .read_to_end(&mut buf)
        .await?;
    assert_eq!(&b"Hello World 1 New"[0..], &buf);

    // remove the item.txt in container 2
    gcs.remove_item(&container_2, "test.txt").await?;
    assert!(gcs.read_item(&container_2, "test.txt").await.is_err());

    // remove the container
    gcs.remove_container(&container_2).await?;

    // remove the item.txt in container 1
    gcs.remove_item(&container_1, "test.txt").await?;
    assert!(gcs.read_item(&container_1, "test.txt").await.is_err());
    gcs.remove_container(&container_1).await?;

    Ok(())
}

async fn reader(data: &str) -> stow::Result<tokio::io::DuplexStream> {
    let (mut send, recv) = tokio::io::duplex(data.len());
    send.write_all(data.as_bytes()).await?;
    send.shutdown().await?;
    Ok(recv)
}
