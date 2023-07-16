
@_ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E = local_unnamed_addr global <{ i8*, [0 x i8] }> <{ i8* bitcast ({ i64, i64 } (i8, [0 x i8]*, i64)* @_ZN6memchr3x867memrchr6detect17h517e20eb83e5ca39E to i8*), [0 x i8] zeroinitializer }>, align 8

@"\01__imp__ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E" = global i8* bitcast (<{ i8*, [0 x i8] }>* @_ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E to i8*)


; memchr::x86::memrchr::detect
; Function Attrs: uwtable
define internal { i64, i64 } @_ZN6memchr3x867memrchr6detect17h517e20eb83e5ca39E(i8 %n1, [0 x i8]* noalias nonnull readonly align 1 %haystack.0, i64 %haystack.1) unnamed_addr #2 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %0 = load atomic i64, i64* getelementptr inbounds ([2 x %"std::std_detect::detect::cache::Cache"], [2 x %"std::std_detect::detect::cache::Cache"]* @_ZN3std10std_detect6detect5cache5CACHE17h36155bdd5a985257E, i64 0, i64 0, i32 1, i32 1) seq_cst, align 8
  %1 = icmp eq i64 %0, -1
  br i1 %1, label %bb7.i.i.i, label %_ZN3std10std_detect6detect4arch21__is_feature_detected4avx217hc0fbd12b9336552cE.exit

bb7.i.i.i:                                        ; preds = %start
; call std::std_detect::detect::os::detect_features
  %2 = tail call i64 @_ZN3std10std_detect6detect2os15detect_features17h30d1c684eca000e9E()
  %_6.i.i.i.i.i = and i64 %2, 9223372036854775807
  store atomic i64 %_6.i.i.i.i.i, i64* getelementptr inbounds ([2 x %"std::std_detect::detect::cache::Cache"], [2 x %"std::std_detect::detect::cache::Cache"]* @_ZN3std10std_detect6detect5cache5CACHE17h36155bdd5a985257E, i64 0, i64 0, i32 1, i32 1) seq_cst, align 8
  %_15.i.i.i.i.i = lshr i64 %2, 63
  store atomic i64 %_15.i.i.i.i.i, i64* getelementptr inbounds ([2 x %"std::std_detect::detect::cache::Cache"], [2 x %"std::std_detect::detect::cache::Cache"]* @_ZN3std10std_detect6detect5cache5CACHE17h36155bdd5a985257E, i64 0, i64 1, i32 1, i32 1) seq_cst, align 8
  br label %_ZN3std10std_detect6detect4arch21__is_feature_detected4avx217hc0fbd12b9336552cE.exit

_ZN3std10std_detect6detect4arch21__is_feature_detected4avx217hc0fbd12b9336552cE.exit: ; preds = %start, %bb7.i.i.i
  %3 = load atomic i64, i64* getelementptr inbounds ([2 x %"std::std_detect::detect::cache::Cache"], [2 x %"std::std_detect::detect::cache::Cache"]* @_ZN3std10std_detect6detect5cache5CACHE17h36155bdd5a985257E, i64 0, i64 0, i32 1, i32 1) seq_cst, align 8
  %4 = trunc i64 %3 to i16
  %5 = icmp slt i16 %4, 0
  %.sink = select i1 %5, i64 ptrtoint ({ i64, i64 } (i8, [0 x i8]*, i64)* @_ZN6memchr3x863avx7memrchr17h082fa08fb3cf7b05E to i64), i64 ptrtoint ({ i64, i64 } (i8, [0 x i8]*, i64)* @_ZN6memchr3x864sse27memrchr17h62035a598a168baeE to i64)
  %6 = select i1 %5, { i64, i64 } (i8, [0 x i8]*, i64)* @_ZN6memchr3x863avx7memrchr17h082fa08fb3cf7b05E, { i64, i64 } (i8, [0 x i8]*, i64)* @_ZN6memchr3x864sse27memrchr17h62035a598a168baeE
  store atomic i64 %.sink, i64* bitcast (<{ i8*, [0 x i8] }>* @_ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E to i64*) monotonic, align 8
  %7 = tail call { i64, i64 } %6(i8 %n1, [0 x i8]* noalias nonnull readonly align 1 %haystack.0, i64 %haystack.1)
  ret { i64, i64 } %7
}
