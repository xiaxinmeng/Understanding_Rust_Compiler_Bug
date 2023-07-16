
error[E0433]: failed to resolve: use of undeclared crate or module `quiche`
  --> src/config.rs:56:30
   |
56 |     let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();
   |                              ^^^^^^ not found in `quiche`
   |
help: consider importing this struct
   |
1  | use crate::Config;
   |
help: if you import `Config`, refer to it directly
   |
56 -     let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();
56 +     let mut config = Config::new(quiche::PROTOCOL_VERSION).unwrap();
   | 

error: future cannot be sent between threads safely
   --> src/connection.rs:78:22
    |
78  |           tokio::spawn(WriteWorker::run(
    |  ______________________^
79  | |             Arc::clone(&conn.transport),
80  | |             flush_notify,
81  | |             writer_cfg,
82  | |             conns,
83  | |             audit_log_stats,
84  | |         ));
    | |_________^ future returned by `run` is not `Send`
    |
    = help: within `ConnectionMap<A>`, the trait `std::marker::Send` is not implemented for `NonNull<u8>`
note: future is not `Send` as this value is used across an await
   --> src/io_workers.rs:109:37
    |
87  |         conns: ConnectionPool<A>,
    |         ----- has type `Arc<std::sync::RwLock<ConnectionMap<A>>>` which is not `Send`
...
109 |         worker.write_loop(transport).await;
    |                                     ^^^^^^ await occurs here, with `conns` maybe used later
110 |     }
    |     - `conns` is later dropped here
note: required by a bound in `tokio::spawn`
   --> /home/jnelson/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.18.2/src/task/spawn.rs:127:21
    |
127 |         T: Future + Send + 'static,
    |                     ^^^^ required by this bound in `tokio::spawn`
