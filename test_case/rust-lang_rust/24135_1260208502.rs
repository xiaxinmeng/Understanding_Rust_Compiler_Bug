rust
impl Child {
  pub async fn wait(&mut self) -> std::io::Result<ExitStatus>;
}
