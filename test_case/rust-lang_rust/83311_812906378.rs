
thread '<unnamed>' panicked at 'Error reading cached dep-graph: "invalid enum variant tag while decoding `DepKind`, expected 0..249"', compiler/rustc_incremental/src/persist/load.rs:186:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z query-dep-graph -C prefer-dynamic -C rpath -C debuginfo=0 -C link-args=-fuse-ld=lld -C incremental

query stack during panic:
end of query stack
warning: could not decode incremental cache: Any

warning: 1 warning emitted
