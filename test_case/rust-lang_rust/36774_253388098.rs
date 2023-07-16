
$ rustc -Vv
rustc 1.14.0-nightly (a3bc191b5 2016-10-10)
binary: rustc
commit-hash: a3bc191b5f41df5143cc65084b13999896411817
commit-date: 2016-10-10
host: x86_64-unknown-linux-gnu
release: 1.14.0-nightly
$ cargo -Vv
cargo 0.13.0-nightly (957a47b 2016-10-10)
$ git rev-parse HEAD
ee3acca1035f41d87a6be91b42daf02c565ae996
$ RUSTFLAGS=-g cargo build --release
[...]
   Compiling cargo v0.14.0 (file:///home/jistone/rust/cargo)
piece is larger than or outside of variable
  tail call void @llvm.dbg.value(metadata i8* %2, i64 0, metadata !46311, metadata !2682), !dbg !46315
!46311 = !DILocalVariable(arg: 4, scope: !46265, file: !10, line: 1, type: !46295)
!2682 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  tail call void @llvm.dbg.value(metadata i64 %3, i64 0, metadata !46311, metadata !2679), !dbg !46315
!46311 = !DILocalVariable(arg: 4, scope: !46265, file: !10, line: 1, type: !46295)
!2679 = !DIExpression(DW_OP_bit_piece, 64, 64)
piece is larger than or outside of variable
  tail call void @llvm.dbg.value(metadata i32 %5, i64 0, metadata !46311, metadata !46317), !dbg !46315
!46311 = !DILocalVariable(arg: 4, scope: !46265, file: !10, line: 1, type: !46295)
!46317 = !DIExpression(DW_OP_bit_piece, 256, 32)
LLVM ERROR: Broken function found, compilation aborted!
error: Could not compile `cargo`.
