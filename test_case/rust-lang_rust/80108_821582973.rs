
PS> cargo build --target wasm32-unknown-unknown
   Compiling example v0.1.0 (C:\Users\${name}\project\example)
Invalid bitcast
  %4 = bitcast <4 x i32> %1 to [4 x i32], !dbg !21
in function _ZN7example6Vector8to_array17he7a4eb6f9a744f2cE
LLVM ERROR: Broken function found, compilation aborted!
error: could not compile `example`
