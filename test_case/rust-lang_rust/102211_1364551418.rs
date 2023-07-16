
error: higher-ranked lifetime error
  --> src/lib.rs:25:18
   |
25 |     let handle = tokio::spawn(accept_connection(rpc));
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: could not prove `impl futures::Future<Output = Result<(), anyhow::Error>>: std::marker::Send
