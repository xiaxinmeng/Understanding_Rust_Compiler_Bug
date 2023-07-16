
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:755:13: Broken MIR: generator contains type &mut Body in MIR, but typeck only knows about {ResumeTy, &HttpClient, std::time::Duration, HttpClient, &Client<HttpConnector>, Client<HttpConnector>, Request<Body>, tokio::time::Timeout<hyper::client::ResponseFuture>, (), Result<Result<Response<Body>, hyper::Error>, Elapsed>, Response<Body>, Vec<u8>, Next<Body>} and [&request::HttpClient, hyper::Request<hyper::Body>, std::time::Duration]
  --> /root/.cargo/registry/src/github.com-1ecc6299db9ec823/rusoto_credential-0.47.0/src/request.rs:34:99
   |
34 |       pub async fn request(&self, req: Request<Body>, timeout: Duration) -> Result<String, IoError> {
   |  ___________________________________________________________________________________________________^
35 | |         match time::timeout(timeout, self.inner.request(req)).await {
36 | |             Err(_elapsed) => Err(IoError::new(ErrorKind::TimedOut, "Request timed out")),
37 | |             Ok(try_resp) => {
...  |
52 | |         }
53 | |     }
   | |_____^
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/17d29dcdce9b9e838635eb0adefd9b8b1588410b/compiler/rustc_errors/src/lib.rs:1115:9

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (17d29dcdc 2022-01-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z sanitizer=address -C opt-level=3 -C embed-bitcode=no -C passes=sancov-module -C llvm-args=-sanitizer-coverage-level=4 -C llvm-args=-sanitizer-coverage-trace-compares -C llvm-args=-sanitizer-coverage-inline-8bit-counters -C llvm-args=-sanitizer-coverage-pc-table -C link-dead-code -C llvm-args=-sanitizer-coverage-stack-depth -C debug-assertions -C codegen-units=1 --crate-type lib
