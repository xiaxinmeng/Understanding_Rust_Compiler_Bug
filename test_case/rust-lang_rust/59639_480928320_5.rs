
$ rustc +nightly-2019-04-05 --emit=llvm-ir issue48229.rs && grep -C3 trap issue48229.ll
; Function Attrs: noreturn nonlazybind uwtable
define internal void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h0f0e4c14c5a910afE"() unnamed_addr #3 {
start:
  call void @llvm.trap()
  unreachable
}

--
declare void @_ZN4core9panicking9panic_fmt17h48d5842cb1c97579E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48), { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24)) unnamed_addr #2

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #8

; Function Attrs: nonlazybind
define i32 @main(i32, i8**) unnamed_addr #9 {
