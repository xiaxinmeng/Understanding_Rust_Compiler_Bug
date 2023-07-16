
; std::collections::hash::table::calculate_offsets
; Function Attrs: inlinehint uwtable
define internal void @_ZN3std11collections4hash5table17calculate_offsets17hccd439e3af053e99E({ i64, [0 x i8], i64, [0 x i8], i8, [7 x i8] }* noalias nocapture sret dereferenceable(24), i64, i64, i64) unnamed_addr #0 {
start:
  %tmp_ret = alloca { i64, [0 x i8], i8, [7 x i8] }
; call std::collections::hash::table::round_up_to_next
  %4 = call i64 @_ZN3std11collections4hash5table16round_up_to_next17hb5dd1061b779aff5E(i64 %1, i64 %3)
  ...
