plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/ty/context.rs:553:13: node_type: no type for node `infer _ (hir_id=HirId { owner: DefId(0:1966 ~ core[121d]::ptr::metadata::{impl#13}::hash), local_id: 9 })`
thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (667a4b6f2 2021-04-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [check_mod_privacy] checking privacy in module `ptr::metadata`
#1 [analysis] running analysis passes on this crate
error: aborting due to previous error

error: could not compile `core`

