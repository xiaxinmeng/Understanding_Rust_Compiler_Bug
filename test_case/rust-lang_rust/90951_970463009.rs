plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `&str`,
 right: `str`', /checkout/compiler/rustc_middle/src/ty/consts.rs:227:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (f81a44eea 2021-11-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=legacy -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [check_match] match-checking `str::traits::<impl at library/core/src/str/traits.rs:557:1: 590:2>::from_str`
#1 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:05:36
