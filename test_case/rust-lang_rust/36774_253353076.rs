
piece covers entire variable
  call void @llvm.dbg.value(metadata i64 %1, i64 0, metadata !17951, metadata !31648), !dbg !37787
!17951 = !DILocalVariable(arg: 3, scope: !17940, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bb7}")
!31648 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.value(metadata i64 %2, i64 0, metadata !17951, metadata !31649), !dbg !37787
!17951 = !DILocalVariable(arg: 3, scope: !17940, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bb7}")
!31649 = !DIExpression(DW_OP_bit_piece, 64, 64)
piece covers entire variable
  call void @llvm.dbg.value(metadata i64 %1, i64 0, metadata !18006, metadata !31648), !dbg !37803
!18006 = !DILocalVariable(arg: 3, scope: !17997, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bb1}")
!31648 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.value(metadata i64 %2, i64 0, metadata !18006, metadata !31649), !dbg !37803
!18006 = !DILocalVariable(arg: 3, scope: !17997, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bb1}")
!31649 = !DIExpression(DW_OP_bit_piece, 64, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.declare(metadata %"2.std::option::Option<diff::DiffHunk<'static>>"* %arg1.sroa.2, metadata !18065, metadata !37817), !dbg !37818
!18065 = !DILocalVariable(arg: 4, scope: !18055, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bad}")
!37817 = !DIExpression(DW_OP_bit_piece, 64, 128)
piece covers entire variable
  call void @llvm.dbg.value(metadata i64 %1, i64 0, metadata !18065, metadata !31648), !dbg !37818
!18065 = !DILocalVariable(arg: 4, scope: !18055, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bad}")
!31648 = !DIExpression(DW_OP_bit_piece, 0, 64)
piece is larger than or outside of variable
  call void @llvm.dbg.value(metadata i64 %3, i64 0, metadata !18065, metadata !37820), !dbg !37818
!18065 = !DILocalVariable(arg: 4, scope: !18055, file: !10283, line: 1987, type: !"{struct git2/5d1ef9158b6e807755484050a668f7de0ad4666add46b252de68fc8c4e54b3e1/bad}")
!37820 = !DIExpression(DW_OP_bit_piece, 192, 64)
LLVM ERROR: Broken module found, compilation aborted!
error: Could not compile `git2`.
