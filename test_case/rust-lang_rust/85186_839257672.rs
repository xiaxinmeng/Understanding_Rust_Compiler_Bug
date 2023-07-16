plain
.................................................................................................... 9500/11847
.................................................................................................... 9600/11847
........................................................i......i.................................... 9700/11847
.................................................................................................... 9800/11847
..iiiiiii..iiiiii.i................................................................................. 9900/11847
.................................................................................................... 10100/11847
.................................................................................................... 10200/11847
.................................................................................................... 10300/11847
.................................................................................................... 10400/11847
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.188 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.89s

 finished in 3.967 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling tracing-subscriber v0.2.16
   Compiling tracing-tree v0.1.9
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
thread 'rustc' panicked at 'invoked `update_reached_depth` with something under this stack: self.depth=13 reached_depth=19', compiler/rustc_trait_selection/src/traits/select/mod.rs:2164:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (9c085a116 2021-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `std::mem::MaybeUninit<(rustc_span::def_id::DefId, clean::types::TraitWithExtraInfo)>: std::marker::Unpin`
#1 [is_unpin_raw] computing whether `std::mem::MaybeUninit<(rustc_span::def_id::DefId, clean::types::TraitWithExtraInfo)>` is `Unpin`
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.

