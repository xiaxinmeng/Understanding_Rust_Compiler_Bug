 rust
// myapp.rs
logging!(
  handlers: {
    stdout: std::logging::handlers::Stdout::new(),
    logly: myapp::logging::Logly::new()
  }
  loggers: {
    myapp: {},
    mysql2: {}
  }
);
