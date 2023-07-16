
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:755:13: Broken MIR: generator contains type Never in MIR, but typeck only knows about {std::future::ResumeTy, bool, u32, impl std::future::Future<Output = [async output]>, (), impl std::future::Future<Output = [async output]>} and [bool, u32]
  --> $DIR/async-fn.rs:31:53
   |
31 |   async fn includes_never(crash: bool, x: u32) -> u32 {
   |  _____________________________________________________^
32 | |     let mut result = async { x * x }.await;
33 | |     if !crash {
34 | |         return result;
...  |
40 | |     result
41 | | }
   | |_^
