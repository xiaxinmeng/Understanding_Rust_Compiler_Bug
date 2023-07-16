rust
fn send<T: Serialize>(&self, body: &T) -> impl Future<Output=Result<Response, Error>> + 'static {
    let prepared_body: Vec<u8> = serde_json::to_vec(body);
    // We don't need `body` anymore... but rust thinks we do.
    async move {
        let resp = self.client.send(prepared_body?).await?;
        Ok(serde_json::from_slice(&resp)?)
    }
}
