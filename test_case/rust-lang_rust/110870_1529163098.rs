
%0 = load <64 x i8>, ptr %x, align 1, !dbg !11
%.fr = freeze <64 x i8> %0, !dbg !11
%1 = icmp ne <64 x i8> %.fr, <i8 6, i8 4, i8 -120, i8 55, i8 -119, i8 82, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0>, !dbg !11
%2 = bitcast <64 x i1> %1 to i64, !dbg !11
%3 = icmp eq i64 %2, 0, !dbg !11
ret i1 %3, !dbg !12
