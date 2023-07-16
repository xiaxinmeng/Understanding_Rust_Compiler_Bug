plain
   Compiling rand_chacha v0.2.2
   Compiling rand v0.8.4
   Compiling md-5 v0.9.1
   Compiling sha2 v0.9.1
error: internal compiler error: compiler/rustc_mir_transform/src/elaborate_drops.rs:315:25: drop of untracked, uninitialized value bb73, place ((*(_1.0: &mut compile::Compiler)).6: std::option::Option<regex_syntax::utf8::Utf8Sequences>) (Parent(Some(mp133)))
     |
     |
1001 |         self.c.utf8_seqs = Some(utf8_seqs);

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1115:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (4b0c253ff 2021-11-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=v0 -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `compile::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.5.4/src/compile.rs:963:1: 1052:2>::compile`
#1 [optimized_mir] optimizing MIR for `compile::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/regex-1.5.4/src/compile.rs:963:1: 1052:2>::compile`
error: could not compile `regex`
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:11
