none
op@VBOX ~> rustc --version
rustc 1.25.0-nightly (e6072a7b3 2018-01-13)
op@VBOX ~> cargo build --verbose
   Compiling foo v0.1.0 (file:///tmp/tmp.ZxxWikB5JA/foo)
     Running `rustc --crate-name foo src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=76c06508f005ca4a -C extra-filename=-76c06508f005ca4a --out-dir /tmp/tmp.ZxxWikB5JA/foo/target/debug/deps -C incremental=/tmp/tmp.ZxxWikB5JA/foo/target/debug/incremental -L dependency=/tmp/tmp.ZxxWikB5JA/foo/target/debug/deps`
error: Could not compile `foo`.

Caused by:
  process didn't exit successfully: `rustc --crate-name foo src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=76c06508f005ca4a -C extra-filename=-76c06508f005ca4a --out-dir /tmp/tmp.ZxxWikB5JA/foo/target/debug/deps -C incremental=/tmp/tmp.ZxxWikB5JA/foo/target/debug/incremental -L dependency=/tmp/tmp.ZxxWikB5JA/foo/target/debug/deps` (signal: 11, SIGSEGV: invalid memory reference)
