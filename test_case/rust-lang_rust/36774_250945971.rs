
piece covers entire variable
  call void @llvm.dbg.value(metadata i64 %1, i64 0, metadata !10414, metadata !31684), !dbg !34307
!10414 = !DILocalVariable(arg: 3, scope: !10403, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/b9f}")
!31684 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.value(metadata i64 %2, i64 0, metadata !10414, metadata !31685), !dbg !34307
!10414 = !DILocalVariable(arg: 3, scope: !10403, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/b9f}")
!31685 = !DIExpression(DW_OP_bit_piece, 64, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.declare(metadata %"2.std::option::Option<diff::DiffHunk>"* %arg1.sroa.2, metadata !10429, metadata !34312), !dbg !34313
!10429 = !DILocalVariable(arg: 4, scope: !10418, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/b9b}")
!34312 = !DIExpression(DW_OP_bit_piece, 64, 128)
piece covers entire variable
  call void @llvm.dbg.value(metadata i64 %1, i64 0, metadata !10429, metadata !31684), !dbg !34313
!10429 = !DILocalVariable(arg: 4, scope: !10418, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/b9b}")
!31684 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.value(metadata i64 %3, i64 0, metadata !10429, metadata !34315), !dbg !34313
!10429 = !DILocalVariable(arg: 4, scope: !10418, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/b9b}")
!34315 = !DIExpression(DW_OP_bit_piece, 192, 64)
piece covers entire variable
  call void @llvm.dbg.value(metadata i64 %1, i64 0, metadata !10443, metadata !31684), !dbg !34318
!10443 = !DILocalVariable(arg: 3, scope: !10433, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/ba5}")
!31684 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.value(metadata i64 %2, i64 0, metadata !10443, metadata !31685), !dbg !34318
!10443 = !DILocalVariable(arg: 3, scope: !10433, file: !10404, line: 1977, type: !"{struct git2/bd2b4ed1a06da7fa58e6c6d8f6516e398bfeede5c10924aaad59e63a932b47a7/ba5}")
!31685 = !DIExpression(DW_OP_bit_piece, 64, 64)
LLVM ERROR: Broken module found, compilation aborted!
Build failed, waiting for other jobs to finish...
error: Could not compile `git2`.
Caused by:
  Process didn't exit successfully: `rustc .cargo/registry/src/-059c64927a508553/git2-0.4.4/src/lib.rs --crate-name git2 --crate-type lib -C opt-level=3 --cfg feature="default" --cfg feature="https" --cfg feature="ssh" --cfg feature="libgit2-sys" -C metadata=6ff6b7061c01db01 -C extra-filename=-6ff6b7061c01db01 --out-dir /builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps --emit=dep-info,link --target x86_64-unknown-linux-gnu -L dependency=/builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps -L dependency=/builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps --extern url=/builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps/liburl-bf7ff6dce96f3de6.rlib --extern libc=/builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps/liblibc-d7f563bb6cbdc70a.rlib --extern bitflags=/builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps/libbitflags-877983b26a517965.rlib --extern libgit2_sys=/builddir/build/BUILD/cargo-0.13.0/target/x86_64-unknown-linux-gnu/release/deps/liblibgit2_sys-c6ac804eabc4dcd5.rlib --cap-lints allow -L native=/usr/lib64 -L native=/usr/lib64 -L native=/usr/lib64 -L native=/usr/lib64 -C opt-level=3 -g` (exit code: 1)
