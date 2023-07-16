
define void @_ZN7example3foo17h9f11ae7042742a8dE(ptr noalias nocapture noundef align 8 dereferenceable(24) %t) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %taken.sroa.6.0.t.sroa_idx = getelementptr inbounds i8, ptr %t, i64 8
  %taken.sroa.6.0.copyload5 = load i64, ptr %taken.sroa.6.0.t.sroa_idx, align 8, !alias.scope !2, !noalias !6
  %taken.sroa.7.0.t.sroa_idx = getelementptr inbounds i8, ptr %t, i64 16
  %taken.sroa.7.0.copyload6 = load i64, ptr %taken.sroa.7.0.t.sroa_idx, align 8, !alias.scope !2, !noalias !6
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %taken.sroa.6.0.t.sroa_idx, i8 0, i64 16, i1 false)
  %0 = icmp eq i64 %taken.sroa.7.0.copyload6, 0
  %1 = add i64 %taken.sroa.7.0.copyload6, -1
  %spec.select = select i1 %0, i64 0, i64 %1
  store i64 %taken.sroa.6.0.copyload5, ptr %taken.sroa.6.0.t.sroa_idx, align 8
  store i64 %spec.select, ptr %taken.sroa.7.0.t.sroa_idx, align 8
  ret void
}
