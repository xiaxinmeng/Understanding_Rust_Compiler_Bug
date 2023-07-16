rust
// async fn foo_async($parameter_patterns) { $body }
fn foo_async(raw_parameters) {
  async move {
    let $parameter_patterns = raw-parameters;
    return $body; // body of the async fn
  }
}
