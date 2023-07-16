rust
// with `E1: From<E2>`
result.map(|res| res.map_err(anyhow::Error::from)).flatten_into()?;
// with `E2: From<E1>`
result.map_err(anyhow::Error::from).flatten_into()?;
