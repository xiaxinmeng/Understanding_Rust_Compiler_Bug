rust
use anyhow::Result;
use websocket;

fn main() -> Result<()> {
    let session = Session::new()?;
    Ok(())
}

struct Session {
    client: websocket::sync::Client<impl websocket::sync::Stream>
}

impl Session {
	fn new() -> Result<Self> {
	    let client = websocket::ClientBuilder::new("ws://...")?.connect_insecure()?;
	    Ok(Session { client })
	}
}
