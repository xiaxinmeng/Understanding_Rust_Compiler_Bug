
*** IR Dump After InstCombinePass on _ZN4test4test17h60e5e3f4ec590b13E ***
; Function Attrs: nonlazybind uwtable
define { i64, i64 } @_ZN4test4test17h60e5e3f4ec590b13E(i64 %x.0, i64 %x.1) unnamed_addr #0 {
start:
  %0 = call fastcc { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17hed04f6fc65ae78f0E"(i64 %x.0, i64 %x.1)
  %.fca.0.extract1 = extractvalue { i64, i64 } %0, 0
  %.fca.1.extract2 = extractvalue { i64, i64 } %0, 1
  switch i64 %.fca.0.extract1, label %bb3 [
    i64 0, label %bb2
    i64 1, label %bb4
  ]

bb3:                                              ; preds = %start
  unreachable

bb2:                                              ; preds = %start
  br label %bb6

bb4:                                              ; preds = %start
  %1 = call fastcc i64 @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h7fc30301c010dde1E"(i64 %.fca.1.extract2)
  br label %bb6

bb6:                                              ; preds = %bb2, %bb4
  %.sroa.3.0 = phi i64 [ %1, %bb4 ], [ %.fca.1.extract2, %bb2 ]
  %.sroa.0.0 = phi i64 [ 1, %bb4 ], [ 0, %bb2 ]
  %2 = insertvalue { i64, i64 } undef, i64 %.sroa.0.0, 0
  %3 = insertvalue { i64, i64 } %2, i64 %.sroa.3.0, 1
  ret { i64, i64 } %3
}
*** IR Dump After SimplifyCFGPass on _ZN4test4test17h60e5e3f4ec590b13E ***
; Function Attrs: nonlazybind uwtable
define { i64, i64 } @_ZN4test4test17h60e5e3f4ec590b13E(i64 %x.0, i64 %x.1) unnamed_addr #0 {
start:
  %0 = call fastcc { i64, i64 } @"_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17hed04f6fc65ae78f0E"(i64 %x.0, i64 %x.1)
  %.fca.0.extract1 = extractvalue { i64, i64 } %0, 0
  %.fca.1.extract2 = extractvalue { i64, i64 } %0, 1
  %switch = icmp ult i64 %.fca.0.extract1, 1
  br i1 %switch, label %bb6, label %bb4

bb4:                                              ; preds = %start
  %1 = call fastcc i64 @"_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h7fc30301c010dde1E"(i64 %.fca.1.extract2)
  br label %bb6

bb6:                                              ; preds = %start, %bb4
  %.sroa.3.0 = phi i64 [ %1, %bb4 ], [ %.fca.1.extract2, %start ]
  %.sroa.0.0 = phi i64 [ 1, %bb4 ], [ 0, %start ]
  %2 = insertvalue { i64, i64 } undef, i64 %.sroa.0.0, 0
  %3 = insertvalue { i64, i64 } %2, i64 %.sroa.3.0, 1
  ret { i64, i64 } %3
}
