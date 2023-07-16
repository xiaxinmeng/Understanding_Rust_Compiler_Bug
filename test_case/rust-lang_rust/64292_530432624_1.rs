rust
let msg = format!("from {}: {}\n", from, msg);
peer.send(msg).await?
