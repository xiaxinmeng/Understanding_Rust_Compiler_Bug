llvm
  %0 = load <2 x i8>, ptr %s1, align 1, !dbg !11
  %1 = load <2 x i8>, ptr %s2, align 1, !dbg !12
  %2 = icmp eq <2 x i8> %0, %1, !dbg !11
  %3 = extractelement <2 x i1> %2, i64 0, !dbg !11
  %4 = extractelement <2 x i1> %2, i64 1, !dbg !11
