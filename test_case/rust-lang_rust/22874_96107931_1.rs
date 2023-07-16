
~/Downloads $ rustc test.rs
Call parameter type does not match function signature!
  %6 = phi %Value* [ %3, %entry-block ], [ %8, %iter_vec_loop_body ]
 { %Value*, i64 }*  call void @"_ZN16_$u5b$Value$u5d$8drop.95817h5f8aee98e1349761E"(%Value* %6)
LLVM ERROR: Broken function found, compilation aborted!

~/Downloads $ rustc --version
rustc 1.0.0-beta.2 (e9080ec39 2015-04-16) (built 2015-04-16)
