 llvm
define void @_rust_main({ i64, %tydesc*, i8*, i8*, i8 }* nocapture) {
"function top level":
  %1 = tail call fastcc i64 @"_ZN9pass_343215_927cf3b1d8737b7_0$x2e0E"()
  %2 = icmp sgt i64 %1, 0
  br i1 %2, label %match_else.i, label %"_ZN4main17_15bec6257b8bff787_0$x2e0E.exit"

match_else.i:                                     ; preds = %"function top level", %match_else.i
  %..sroa.0.023.i = phi i64 [ %..sroa.0.0.i, %match_else.i ], [ 1, %"function top level" ]
  %.sroa.0.022.i = phi i64 [ %..sroa.0.023.i, %match_else.i ], [ 0, %"function top level" ]
  %.sroa.3.021.i = phi i64 [ %3, %match_else.i ], [ 0, %"function top level" ]
  %3 = add i64 %.sroa.3.021.i, 1
  tail call fastcc void @"_ZN8out_402115_136b6da5d48b8d7_0$x2e0E"(i64 %.sroa.0.022.i)
  tail call fastcc void @"_ZN8out_402115_136b6da5d48b8d7_0$x2e0E"(i64 %.sroa.3.021.i)
  %4 = icmp slt i64 %..sroa.0.023.i, %1
  %5 = zext i1 %4 to i64
  %..sroa.0.0.i = add i64 %5, %..sroa.0.023.i
  %.not.i = icmp sgt i64 %3, 19
  %.not19.i = xor i1 %4, true
  %brmerge.i = or i1 %.not.i, %.not19.i
  br i1 %brmerge.i, label %"_ZN4main17_15bec6257b8bff787_0$x2e0E.exit", label %match_else.i

"_ZN4main17_15bec6257b8bff787_0$x2e0E.exit":      ; preds = %match_else.i, %"function top level"
  ret void
}
