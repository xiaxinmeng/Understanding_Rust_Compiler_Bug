plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0599]: no function or associated item named `parse_inner` found for struct `config::Config` in the current scope
  --> config/tests.rs:10:13
   |
10 |     Config::parse_inner(&["check".to_owned(), "--config=/does/not/exist".to_owned()], toml(config))
   |
  ::: config.rs:60:1
   |
60 | pub struct Config {
