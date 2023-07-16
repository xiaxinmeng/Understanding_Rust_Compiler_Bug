 llvm
match_case.i:                                     ; preds = %"_ZN5slice57Items$LT$$x27a$C$$x20T$GT$.Iterator$LT$$BP$$x27a$x20T$GT$4next21h11506221690941188551E.exit"
  %sret_slot.sroa.0.0.i.lcssa310 = phi i8* [ %sret_slot.sroa.0.0.i, %"_ZN5slice57Items$LT$$x27a$C$$x20T$GT$.Iterator$LT$$BP$$x27a$x20T$GT$4next21h11506221690941188551E.exit" ]
  %60 = icmp eq i8* %sret_slot.sroa.0.0.i.lcssa310, null
  br i1 %60, label %.noexc76, label %then-block-191-.i.loopexit309

match_case6.i:                                    ; preds = %"_ZN5slice57Items$LT$$x27a$C$$x20T$GT$.Iterator$LT$$BP$$x27a$x20T$GT$4next21h11506221690941188551E.exit"
  %61 = icmp eq i8* %sret_slot.sroa.0.0.i, null
  br i1 %61, label %then-block-191-.i.loopexit, label %.noexc197

.noexc197:                                        ; preds = %match_case6.i
  %62 = bitcast i8* %sret_slot.sroa.0.0.i to i64*
  %63 = bitcast i8* %sret_slot.sroa.0.0.i201 to i64*
  %64 = load i64* %63, align 8
  %65 = load i64* %62, align 8
  %66 = icmp eq i64 %64, %65
  br i1 %66, label %loop_body.i, label %then-block-191-.i.loopexit
