rust
impl Drop for Client {
    fn drop(&mut self) {}
}
// ...
    let g = move || match status(Client { ..Client::default() }) {
