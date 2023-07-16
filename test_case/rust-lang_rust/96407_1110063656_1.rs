
(truncated)
Dist rls-1.60.0-dev-x86_64-unknown-linux-gnu
	finished in 12.627 seconds
Building stage1 tool rust-analyzer (x86_64-unknown-linux-gnu)
error: failed to select a version for the requirement `anyhow = "^1.0.26"`
candidate versions found which didn't match: 1.0.51
location searched: directory source `<hidden>/rust/vendor` (which is replacing registry `crates-io`)
required by package `xtask v0.1.0 (<hidden>/rust/src/tools/rust-analyzer/xtask)`
perhaps a crate was updated and forgotten to be re-vendored?
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1090:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:11:24
