`
   Compiling chalk-integration v0.62.0-dev.0 (/home/matthias/vcs/github/chalk/chalk-integration)
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(e3239918b04f5ea6-d9d33836ead20deb): Ok(EvaluatedToOk)', /rustc/61edfd591cedff66fca639c02f66984f6271e5a6/compiler/rustc_query_system/src/query/plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (61edfd591 2021-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `chalk_ir::BindersIntoIterator<&std::vec::Vec<chalk_ir::Binders<chalk_ir::WhereClause<interner::ChalkIr>>>>: std::marker::Sized`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [std::iter::Map<chalk_ir::BindersIntoIterator<&std::vec::Vec<chalk_ir::Binders<chalk_ir::WhereClause<interner::ChalkIr>>>>, [closure@<chalk_solve::rust_ir::AssociatedTyValue<interner::ChalkIr> as chalk_solve::clauses::program_clauses::ToProgramClauses<interner::ChalkIr>>::to_program_clauses::{closure#0}::{closure#3}]>], item_def_id: DefId(2:6993 ~ core[3998]::iter::traits::collect::IntoIterator::IntoIter) } } }`
end of query stack
error: could not compile `chalk-integration`

To learn more, run the command again with --verbose.
