
---- node::node_configuration::into_proto_http stdout ----
Error: could not parse connection endpoint: invalid scheme for endpoint: bad-scheme

Caused by:
    invalid scheme for endpoint: bad-scheme
thread 'node::node_configuration::into_proto_http' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: the test returned a termination value with a non-zero status code (1) which indicates a failure', /private/tmp/nix-build-rustc-1.45.2.drv-0/rustc-1.45.2-src/src/libstd/macros.rs:16:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
