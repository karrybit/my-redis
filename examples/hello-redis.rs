use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open connection
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set "world" to "hello"
    client.set("hello", "world".into()).await?;

    // get "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
