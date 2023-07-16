diff
- %_9 = load i64, ptr %self, align 8, !dbg !129, !range !119, !noundef !12
+ %_9 = select i1 %4, i64 0, i64 1, !dbg !147
