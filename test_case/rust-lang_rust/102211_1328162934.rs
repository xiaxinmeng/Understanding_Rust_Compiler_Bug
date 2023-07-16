
error: higher-ranked lifetime error
   --> bot/src/main.rs:141:23
    |
141 |         let handle2 = tokio::task::spawn(robinhood_task());
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: could not prove `impl Future<Output = ()>: Send`

