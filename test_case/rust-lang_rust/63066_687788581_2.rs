rust
use anyhow::Result;
use websocket;

fn main() -> Result<()> {
    let session = create_session()?;
    Ok(())
}

struct Session<T: websocket::sync::Stream> {
    client: websocket::sync::Client<T>,
}

fn create_session() -> Result<Session<impl websocket::sync::Stream>> {
    let client = websocket::ClientBuilder::new("ws://...")?.connect_insecure()?;
    Ok(Session { client })
} 
