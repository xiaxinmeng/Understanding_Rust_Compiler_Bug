rust
async fn foo(<pattern> @ x: Type) {
}

// becomes
fn foo(<pattern> @ x: Type) {
  async move {
    let <pattern> = x;
  } // <-- as you "exit" the async block
}
