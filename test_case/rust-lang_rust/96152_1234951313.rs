llvm
define nonnull ptr @_ZN7example9old_style17h5807fc970566528bE(ptr %a) unnamed_addr #0 {
  %_3 = ptrtoint ptr %a to i64
  %_2 = or i64 %_3, 1
  %0 = inttoptr i64 %_2 to ptr
  ret ptr %0
}

define ptr @_ZN7example12cheri_compat17hf4c9a96c91413294E(ptr %a) unnamed_addr #0 {
  %old = ptrtoint ptr %a to i64
  %old.not = and i64 %old, 1
  %diff = xor i64 %old.not, 1
  %0 = getelementptr i8, ptr %a, i64 %diff
  ret ptr %0
}

define ptr @_ZN7example4fast17hfe789c747db55c0eE(ptr %a) unnamed_addr #0 {
  %old = ptrtoint ptr %a to i64
  %new = or i64 %old, 1
  %count = sub i64 0, %old
  %0 = getelementptr i8, ptr %a, i64 %count
  %1 = getelementptr i8, ptr %0, i64 %new
  ret ptr %1
}
