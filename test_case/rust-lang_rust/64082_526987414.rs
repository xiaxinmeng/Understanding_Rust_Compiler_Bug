llvm
  %0 = icmp ugt i32 %a.val, %b.val, !dbg !9
  %1 = icmp ult i32 %a.val, %b.val, !dbg !12
  %2 = zext i1 %1 to i64, !dbg !9
  %3 = xor i64 %2, -1, !dbg !9
  %.off = sext i1 %0 to i64, !dbg !13
  %switch = icmp eq i64 %3, %.off, !dbg !13
