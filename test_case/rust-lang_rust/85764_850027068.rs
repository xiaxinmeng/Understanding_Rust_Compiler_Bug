plain
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.25
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: internal compiler error: compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs:109:41: we shouldn't actually return an error here
thread 'rustc' panicked at 'Box<Any>', /checkout/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (7b8619042 2021-05-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [specialization_graph_of] building specialization graph of trait `std::convert::From`
#1 [check_mod_impl_wf] checking that impls are well-formed in module `owning_ref`
error: aborting due to previous error

error: could not compile `rustc_data_structures`

