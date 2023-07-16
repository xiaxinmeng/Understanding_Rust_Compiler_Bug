llvm
define zeroext i1 @_ZN9sroa_fail10is_empty_117hfeb3b6a38ecff321E(%"core::slice::Iter<f32>"* noalias nocapture readonly dereferenceable(16)) unnamed_addr #0 {
entry-block:
  %xs.sroa.0.0..sroa_idx = getelementptr inbounds %"core::slice::Iter<f32>", %"core::slice::Iter<f32>"* %0, i64 0, i32 0
  %xs.sroa.0.0.copyload = load float*, float** %xs.sroa.0.0..sroa_idx, align 8, !nonnull !0
  %xs.sroa.4.0..sroa_idx12 = getelementptr inbounds %"core::slice::Iter<f32>", %"core::slice::Iter<f32>"* %0, i64 0, i32 1
  %xs.sroa.4.0.copyload = load float*, float** %xs.sroa.4.0..sroa_idx12, align 8, !nonnull !0
  %1 = icmp eq float* %xs.sroa.0.0.copyload, %xs.sroa.4.0.copyload
  ret i1 %1
}
