
error: internal compiler error: encountered incremental compilation error with evaluate_obligation(da800f1097592f6e-98d6b04e2aecd25a)
  |
  = help: This is a known issue with the compiler. Run `cargo clean -p rtcexperiment` or `cargo clean` to allow your project to compile
  = note: Please follow the instructions below to create a bug report with the provided information
  = note: See <https://github.com/rust-lang/rust/issues/84970> for more information

thread 'rustc' panicked at 'Found unstable fingerprints for evaluate_obligation(da800f1097592f6e-98d6b04e2aecd25a): Ok(EvaluatedToOk)', /rustc/08095fc1f875c89e507f17cf6c6a780c8ffa4c01/compiler/rustc_query_system/src/query/plumbing.rs:624:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/bjorn3/rustc_codegen_cranelift/issues/new

note: rustc 1.56.0-nightly (08095fc1f 2021-07-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `axum::routing::RouteFuture<axum::service::HandleError<tower_http::services::ServeFile, [closure@src/main.rs:18:65: 23:6], hyper::Body>, axum::routing::EmptyRouter, hyper::Body>: std::marker::Sized`
#1 [type_op_prove_predicate] evaluating `type_op_prove_predicate` `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing }, value: ProvePredicate { predicate: Binder(TraitPredicate(<hyper::common::exec::Exec as hyper::common::exec::NewSvcExec<hyper::server::conn::AddrStream, tower::make::make_service::shared::SharedFuture<axum::routing::Route<axum::service::HandleError<tower_http::services::ServeFile, [closure@src/main.rs:18:65: 23:6], hyper::Body>, axum::routing::Route<axum::service::HandleError<tower_http::services::ServeFile, [closure@src/main.rs:18:65: 23:6], hyper::Body>, axum::routing::EmptyRouter>>>, axum::routing::Route<axum::service::HandleError<tower_http::services::ServeFile, [closure@src/main.rs:18:65: 23:6], hyper::Body>, axum::routing::Route<axum::service::HandleError<tower_http::services::ServeFile, [closure@src/main.rs:18:65: 23:6], hyper::Body>, axum::routing::EmptyRouter>>, hyper::common::exec::Exec, hyper::server::conn::spawn_all::NoopWatcher>>), []) } } }`
end of query stack
