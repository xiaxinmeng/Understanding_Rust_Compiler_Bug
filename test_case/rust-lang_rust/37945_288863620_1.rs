llvm
"core::slice::Iter<f32>" = type { float*, float*, %"core::marker::PhantomData<&f32>" }
%"core::marker::PhantomData<&f32>" = type {}
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type {}

; Function Attrs: nounwind uwtable
define zeroext i1 @_ZN3bad10is_empty_117h1c4be4edff235dbfE(%"core::slice::Iter<f32>"* noalias nocapture readonly dereferenceable(16)) unnamed_addr #0 {
entry-block:
  %xs.sroa.0.0..sroa_idx = getelementptr inbounds %"core::slice::Iter<f32>", %"core::slice::Iter<f32>"* %0, i64 0, i32 0
  %xs.sroa.0.0.copyload = load float*, float** %xs.sroa.0.0..sroa_idx, align 8, !nonnull !0
  %xs.sroa.4.0..sroa_idx13 = getelementptr inbounds %"core::slice::Iter<f32>", %"core::slice::Iter<f32>"* %0, i64 0, i32 1
  %xs.sroa.4.0.copyload = load float*, float** %xs.sroa.4.0..sroa_idx13, align 8, !nonnull !0
  %1 = icmp eq float* %xs.sroa.0.0.copyload, %xs.sroa.4.0.copyload
  ret i1 %1
}

; Function Attrs: nounwind uwtable
define zeroext i1 @_ZN3bad10is_empty_217h72beabc012005049E(%"core::slice::Iter<f32>"* noalias nocapture readonly dereferenceable(16)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  %xs.sroa.0.0..sroa_idx = getelementptr inbounds %"core::slice::Iter<f32>", %"core::slice::Iter<f32>"* %0, i64 0, i32 0
  %xs.sroa.0.0.copyload = load float*, float** %xs.sroa.0.0..sroa_idx, align 8, !nonnull !0
  %xs.sroa.4.0..sroa_idx14 = getelementptr inbounds %"core::slice::Iter<f32>", %"core::slice::Iter<f32>"* %0, i64 0, i32 1
  %xs.sroa.4.0.copyload = load float*, float** %xs.sroa.4.0..sroa_idx14, align 8, !nonnull !0
  %1 = icmp eq float* %xs.sroa.0.0.copyload, %xs.sroa.4.0.copyload
  ret i1 %1
}

; Function Attrs: nounwind
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

attributes #0 = { nounwind uwtable }
attributes #1 = { nounwind }

!0 = !{}
