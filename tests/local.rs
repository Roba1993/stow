use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_local() -> stow::Result<()> {
    // create a new environment if not avilable
    let mut local = stow::Location::new_local("./data").await?;

    let container_1 = "container-1";
    let container_2 = "container-2";

    // create new containers if not avilable
    local.create_container(container_1).await?;
    local.create_container(container_2).await?;
    assert!(local
        .containers()
        .await?
        .contains(&String::from(container_1)));
    assert!(local
        .containers()
        .await?
        .contains(&String::from(container_2)));

    // create two test.txt file
    local
        .create_item(container_1, "test.txt", reader("Hello World 1").await?)
        .await?;
    local
        .create_item(container_2, "test.txt", reader("Hello World 2").await?)
        .await?;
    assert!(local
        .items(container_2)
        .await?
        .contains(&String::from("test.txt")));

    // rewrite the test.txt file
    local
        .create_item(container_1, "test.txt", reader("Hello World 1 New").await?)
        .await?;

    // read the test.txt file
    let mut buf = vec![];
    local
        .read_item(container_1, "test.txt")
        .await?
        .read_to_end(&mut buf)
        .await?;
    assert_eq!(&b"Hello World 1 New"[0..], &buf);

    // remove the item.txt in container 2
    local.remove_item(container_2, "test.txt").await?;
    assert!(local.read_item(container_2, "test.txt").await.is_err());

    // remove the container
    local.remove_container(container_2).await?;
    local.remove_container(container_1).await?;

    Ok(())
}

async fn reader(data: &str) -> stow::Result<tokio::io::DuplexStream> {
    let (mut send, recv) = tokio::io::duplex(data.len());
    send.write_all(data.as_bytes()).await?;
    send.shutdown().await?;
    Ok(recv)
}
