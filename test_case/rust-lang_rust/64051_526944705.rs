
 Downloading crates ...
  Downloaded lazy_static v0.2.5
  Downloaded conduit-mime-types v0.7.3
error: failed to download from `https://crates.io/api/v1/crates/log/0.3.7/download`

Caused by:
  [35] SSL connect error (error:14094410:SSL routines:ssl3_read_bytes:sslv3 alert handshake failure)
thread 'main' panicked at 'tests failed for https://github.com/iron/iron', src/tools/cargotest/main.rs:86:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
