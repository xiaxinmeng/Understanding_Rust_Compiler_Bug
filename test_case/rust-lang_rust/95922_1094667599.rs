rust
fn call_service<'a, Req>(req: &'a Req) -> impl Future {
  async move { let x = req; }
}
