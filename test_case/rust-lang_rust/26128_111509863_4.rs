
; Function Attrs: inlinehint nounwind readonly uwtable
define zeroext i1 @_ZN26Test...std..cmp..PartialEq2eq20h54f0ffdc782fbe51maaE(i8* noalias nocapture readonly dereferenceable(1), i8* noalias nocapture readonly dereferenceable(1)) unnamed_addr #0 {
entry-block:
  %2 = load i8* %1, align 1, !range !0
  switch i8 %2, label %join [
    i8 0, label %match_case
    i8 1, label %match_case8
    i8 2, label %match_case11
    i8 3, label %match_case14
    i8 4, label %match_case17
  ]

match_case:                                       ; preds = %entry-block
  %3 = load i8* %0, align 1, !range !0
  %cond23 = icmp eq i8 %3, 0
  br label %join

match_case8:                                      ; preds = %entry-block
  %4 = load i8* %0, align 1, !range !0
  %cond22 = icmp eq i8 %4, 1
  br label %join

match_case11:                                     ; preds = %entry-block
  %5 = load i8* %0, align 1, !range !0
  %cond21 = icmp eq i8 %5, 2
  br label %join

match_case14:                                     ; preds = %entry-block
  %6 = load i8* %0, align 1, !range !0
  %cond20 = icmp eq i8 %6, 3
  br label %join

match_case17:                                     ; preds = %entry-block
  %7 = load i8* %0, align 1, !range !0
  %cond = icmp eq i8 %7, 4
  ret i1 %cond

join:                                             ; preds = %match_case14, %match_case11, %match_case8, %match_case, %entry-block
  %sret_slot.0 = phi i1 [ false, %entry-block ], [ %cond23, %match_case ], [ %cond22, %match_case8 ], [ %cond21, %match_case11 ], [ %cond20, %match_case14 ]
  ret i1 %sret_slot.0
}
