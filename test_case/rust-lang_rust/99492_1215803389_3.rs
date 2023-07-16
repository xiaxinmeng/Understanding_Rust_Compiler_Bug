
error: higher-ranked lifetime error
  --> src/main.rs:22:9
   |
22 | /         tokio::task::spawn(async move {
23 | |             execution_process(&self).await;
24 | |         })
   | |__________^
   |
   = note: could not prove impl futures::Future<Output = ()>: std::marker::Send
