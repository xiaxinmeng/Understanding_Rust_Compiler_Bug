
~/Downloads $ rustc test.rs
...
Stored value type does not match pointer operand type!
  store { %"Sample<f32>"*, i64 }* %0, { %"Sample<f32>"*, i64 }* %to_ref
 { %"Sample<f32>"*, i64 }LLVM ERROR: Broken function found, compilation aborted!
