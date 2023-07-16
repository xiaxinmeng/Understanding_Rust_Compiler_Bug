
define zeroext i1 @example::exists_in_table(i32 %v) unnamed_addr #0 !dbg !12 {
  %0 = icmp eq i32 %v, 0, !dbg !14
  ret i1 %0, !dbg !18
}
