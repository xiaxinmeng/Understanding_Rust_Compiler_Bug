plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_data_structures/src/graph/dominators/mod.rs:53:55

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (761c4b6a5 2021-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=legacy -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `array::<impl at library/core/src/array/mod.rs:340:9: 344:10>::{constant#0}`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `array::<impl at library/core/src/array/mod.rs:340:9: 344:10>::{constant#0}`
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:10
