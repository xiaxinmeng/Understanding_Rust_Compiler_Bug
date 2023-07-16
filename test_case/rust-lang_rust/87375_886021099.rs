plain
   Compiling unicode-width v0.1.8
   Compiling scoped-tls v1.0.0
   Compiling termcolor v1.1.2
   Compiling annotate-snippets v0.8.0
error: internal compiler error: broken MIR in DefId(0:145 ~ smallvec[c47d]::{impl#10}::drop) (NoSolution): could not prove Binder(const TraitPredicate(<<T as Array>::Item as core::marker::Sized>), [])
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (fc6b61044 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `smallvec`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in DefId(0:280 ~ arrayvec[7d35]::arrayvec::{impl#21}::drop) (NoSolution): could not prove Binder(const TraitPredicate(<T as core::marker::Sized>), [])
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (fc6b61044 2021-07-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
