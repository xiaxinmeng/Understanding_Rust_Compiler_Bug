
i~/g/rust-analyzer
λ  (master|✔️ )  rustc --version --verbose                                                                                                                                                                                                                                                                                                 5ms 
rustc 1.52.0-nightly (1705a7d64 2021-03-18)
binary: rustc
commit-hash: 1705a7d64b833d1c4b69958b0627bd054e6d764b
commit-date: 2021-03-18
host: aarch64-apple-darwin
release: 1.52.0-nightly
LLVM version: 12.0.0
i~/g/rust-analyzer
λ  (master|✔️ )  git rev-parse HEAD                                                                                                                                                                                                                                                                                                      139ms 
fc21640a65b5caef8dbbc9e85e9616b843847fb4
i~/g/rust-analyzer
λ  (master|✔️ )  cargo xtask install --mimalloc                                                                                                                                                                                                                                                                                           10ms 
    Finished dev [unoptimized] target(s) in 0.06s
     Running `target/debug/xtask install --mimalloc`
$ cargo install --path crates/rust-analyzer --locked --force --features force-always-assert --features mimalloc
  Installing rust-analyzer v0.0.0 (/Users/benediktmandelkow/gits/rust-analyzer/crates/rust-analyzer)
    Updating crates.io index
   Compiling hir_ty v0.0.0 (/Users/benediktmandelkow/gits/rust-analyzer/crates/hir_ty)
   Compiling hir v0.0.0 (/Users/benediktmandelkow/gits/rust-analyzer/crates/hir)
   Compiling ide_db v0.0.0 (/Users/benediktmandelkow/gits/rust-analyzer/crates/ide_db)
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(3af516d87091aad7-c89d9033dbef420c)', /rustc/1705a7d64b833d1c4b69958b0627bd054e6d764b/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (1705a7d64 2021-03-18) running on aarch64-apple-darwin

note: compiler flags: -C opt-level=3 -C linker-plugin-lto -C split-debuginfo=unpacked -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `RootDatabase: base_db::Upcast<dyn hir::db::AstDatabase>`
#1 [typeck] type-checking `symbol_index::crate_symbols`
end of query stack
error: could not compile `ide_db`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(b804705c3ed9ee58-9edc697b7a408cab)', /rustc/1705a7d64b833d1c4b69958b0627bd054e6d764b/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (1705a7d64 2021-03-18) running on aarch64-apple-darwin

note: compiler flags: -C opt-level=3 -C linker-plugin-lto -C split-debuginfo=unpacked -C debuginfo=0 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `chalk_ir::BindersIntoIterator<&std::vec::Vec<chalk_ir::Binders<chalk_ir::WhereClause<traits::chalk::interner::Interner>>>>: std::marker::Sized`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [std::iter::Map<chalk_ir::BindersIntoIterator<&std::vec::Vec<chalk_ir::Binders<chalk_ir::WhereClause<traits::chalk::interner::Interner>>>>, [closure@<chalk_solve::rust_ir::AssociatedTyValue<traits::chalk::interner::Interner> as chalk_solve::clauses::program_clauses::ToProgramClauses<traits::chalk::interner::Interner>>::to_program_clauses::{closure#0}::{closure#3}]>], item_def_id: DefId(2:7000 ~ core[ae58]::iter::traits::collect::IntoIterator::IntoIter) } } }`
end of query stack
error: failed to compile `rust-analyzer v0.0.0 (/Users/benediktmandelkow/gits/rust-analyzer/crates/rust-analyzer)`, intermediate artifacts can be found at `/Users/benediktmandelkow/gits/rust-analyzer/target`

Caused by:
  build failed
Error: install server

Caused by:
    command `cargo install --path crates/rust-analyzer --locked --force --features force-always-assert --features mimalloc` failed, exit code: 101
