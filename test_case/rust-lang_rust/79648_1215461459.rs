
error: higher-ranked lifetime error
  --> src/main.rs:20:5
   |
20 |     demand_is_send(get_foo())
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: could not prove for<'r, 's> impl futures::Future<Output = Vec<()>>: std::marker::Send
