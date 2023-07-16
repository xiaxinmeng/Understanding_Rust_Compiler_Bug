llvm
; foo::get_output1
; Function Attrs: nonlazybind uwtable
define { i8*, i64 } @_ZN3foo11get_output117haa6e1bf361064f05E([10 x i8]* noalias readonly align 1 dereferenceable(10) %input, i64 %offset) unnamed_addr #0 {
start:
  %0 = icmp ult i64 %offset, 8
  %1 = getelementptr inbounds [10 x i8], [10 x i8]* %input, i64 0, i64 %offset
  %_0.sroa.0.0 = select i1 %0, i8* %1, i8* null
  %2 = insertvalue { i8*, i64 } undef, i8* %_0.sroa.0.0, 0
  %3 = insertvalue { i8*, i64 } %2, i64 3, 1
  ret { i8*, i64 } %3
}

; foo::get_output2
; Function Attrs: nonlazybind uwtable
define { i8*, i64 } @_ZN3foo11get_output217h09c70bf053786004E([10 x i8]* noalias readonly align 1 dereferenceable(10) %input, i64 %offset) unnamed_addr #0 {
start:
  %0 = icmp ult i64 %offset, 8
  br i1 %0, label %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h0c1ade7fc9d72442E.exit", label %bb5

"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h0c1ade7fc9d72442E.exit": ; preds = %start
  %1 = sub i64 10, %offset
  %2 = icmp ult i64 %1, 3
  br i1 %2, label %bb4.i.i.i, label %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17hbbe883cdf661ef0bE.exit"

bb4.i.i.i:                                        ; preds = %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h0c1ade7fc9d72442E.exit"
; call core::slice::slice_index_len_fail
  tail call void @_ZN4core5slice20slice_index_len_fail17h3b63b7b5b8377837E(i64 3, i64 %1)
  unreachable

"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17hbbe883cdf661ef0bE.exit": ; preds = %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h0c1ade7fc9d72442E.exit"
  %3 = getelementptr inbounds [10 x i8], [10 x i8]* %input, i64 0, i64 %offset
  br label %bb5

bb5:                                              ; preds = %start, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17hbbe883cdf661ef0bE.exit"
  %_0.sroa.0.0 = phi i8* [ %3, %"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17hbbe883cdf661ef0bE.exit" ], [ null, %start ]
  %4 = insertvalue { i8*, i64 } undef, i8* %_0.sroa.0.0, 0
  %5 = insertvalue { i8*, i64 } %4, i64 3, 1
  ret { i8*, i64 } %5
}
