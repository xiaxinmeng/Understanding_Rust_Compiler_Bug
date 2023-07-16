
    Checking tide v0.16.0
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type Pin<Box<dyn std::future::Future<Output = std::result::Result<response::Response, http_types::Error>> + std::marker::Send>> in MIR, but typeck only knows about for<'r, 's, 't0> {ResumeTy, &'r F, request::Request<State>, middleware::Next<'s, State>, Pin<Box<(dyn std::future::Future<Output = std::result::Result<response::Response, http_types::Error>> + std::marker::Send + 't0)>>, ()}
  --> /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/tide-0.16.0/src/middleware.rs:35:89
   |
35 |       async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> crate::Result {
   |  _________________________________________________________________________________________^
36 | |         (self)(req, next).await
37 | |     }
   | |_____^

thread 'rustc' panicked at 'Box<Any>', /rustc/4fdac23f3171e2f8864d359a21da600dd3faafc9/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (4fdac23f3 2021-03-31) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `middleware::<impl at /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/tide-0.16.0/src/middleware.rs:24:1: 38:2>::handle::{closure#0}`
end of query stack
error: aborting due to previous error

error: could not compile `tide`
