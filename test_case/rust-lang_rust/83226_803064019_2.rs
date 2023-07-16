
thread 'rustc' panicked at 'found unstable fingerprints for evaluate_obligation(1996d96fad4914f7-2e8343523a99712d)', /rustc/36f1f04f18b89ba4a999bcfd6584663fd6fc1c5d\compiler\rustc_query_system\src\query\plumbing.rs:593:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (36f1f04f1 2021-03-17) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `hyper::Body: std::convert::From<&str>`
#1 [normalize_projection_ty] normalizing `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: ProjectionTy { substs: [futures::future::Map<hyper::Server<hyper::server::conn::AddrIncoming, hyper::service::make::MakeServiceFn<[closure@backend\src\main.rs:348:48: 351:18]>>, [closure@backend\src\main.rs:354:35: 354:61]>], item_def_id: DefId(2:11838 ~ core[f48f]::future::future::Future::Output) } } }`
end of query stack
error: could not compile `blog-backend`
