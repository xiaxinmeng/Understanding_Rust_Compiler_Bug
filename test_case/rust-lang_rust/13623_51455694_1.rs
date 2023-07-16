 llvm
define zeroext i1 @_ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE(i8* noalias nocapture readonly dereferenceable(1), i8* noalias nocapture readonly dereferenceable(1)) unnamed_addr #0 {
entry-block:
  %2 = load i8* %0, align 1, !range !0
  switch i8 %2, label %case_body3 [
    i8 0, label %match_case
    i8 1, label %match_case6
    i8 2, label %match_case9
  ]

case_body3:                                       ; preds = %match_case9, %match_case6, %match_case, %entry-block
  br label %join25

match_case:                                       ; preds = %entry-block
  %3 = load i8* %1, align 1, !range !0
  %cond27 = icmp eq i8 %3, 0
  br i1 %cond27, label %join25, label %case_body3

match_case6:                                      ; preds = %entry-block
  %4 = load i8* %1, align 1, !range !0
  %cond26 = icmp eq i8 %4, 1
  br i1 %cond26, label %join25, label %case_body3

match_case9:                                      ; preds = %entry-block
  %5 = load i8* %1, align 1, !range !0
  %cond = icmp eq i8 %5, 2
  br i1 %cond, label %join25, label %case_body3

join25:                                           ; preds = %match_case9, %match_case6, %match_case, %case_body3
  %__make_return_pointer.0 = phi i1 [ false, %case_body3 ], [ true, %match_case ], [ true, %match_case6 ], [ true, %match_case9 ]
  ret i1 %__make_return_pointer.0
}
