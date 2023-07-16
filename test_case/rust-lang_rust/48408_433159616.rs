rust
   Compiling playground v0.0.1 (/playground)
warning: unreachable expression
 --> src/main.rs:4:29
  |
4 |     let _: Box<Fn(usize)> = box panic!();
  |                             ^^^^^^^^^^^^
  |
  = note: #[warn(unreachable_code)] on by default

AddrSpaceCast result must be a pointer
  %4 = addrspacecast i8* %3 to { {}*, [3 x i64]* }, !dbg !915
LLVM ERROR: Broken function found, compilation aborted!
error: Could not compile `playground`.
