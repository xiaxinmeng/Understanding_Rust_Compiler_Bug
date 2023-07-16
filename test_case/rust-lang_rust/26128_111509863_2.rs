
; Function Attrs: inlinehint nounwind readonly uwtable
define zeroext i1 @_ZN26Test...std..cmp..PartialEq2eq20hcca4a1839d4d2498maaE(i8* noalias nocapture readonly dereferenceable(1), i8* noalias nocapture readonly dereferenceable(1)) unnamed_addr #0 {
entry-block:
  %2 = load i8* %0, align 1, !range !0
  %3 = load i8* %1, align 1, !range !0
  %4 = icmp eq i8 %2, %3
  ret i1 %4
}
