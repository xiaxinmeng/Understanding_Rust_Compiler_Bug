
; this code block gets executed 64 times
%187 = getelementptr inbounds [64 x i8], ptr %x, i64 0, i64 62, !dbg !11
%188 = load i8, ptr %187, align 1, !dbg !11, !noundef !10
%189 = icmp eq i8 %188, 0, !dbg !11
br i1 %189, label %bb63, label %bb64, !dbg !11
