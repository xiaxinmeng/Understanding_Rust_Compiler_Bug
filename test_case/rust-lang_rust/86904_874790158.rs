plain
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (168c95950 2021-07-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `output::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/polonius-engine-0.12.1/src/output/mod.rs:142:1: 463:2>::compute`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: could not compile `polonius-engine`
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:22
