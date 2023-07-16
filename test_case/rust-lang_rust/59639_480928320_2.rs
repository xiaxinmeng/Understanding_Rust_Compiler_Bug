
$ rustc +nightly-2019-04-05 --emit=llvm-ir issue48227.rs && grep -C3 trap issue48227.ll
; Function Attrs: noreturn nonlazybind uwtable
define void @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h8496458b8d57254eE"() unnamed_addr #3 {
start:
  call void @llvm.trap()
  unreachable
}

--
declare zeroext i1 @"_ZN42_$LT$$u21$$u20$as$u20$core..fmt..Debug$GT$3fmt17h48bee2ab31956489E"({ [0 x i8] }* noalias nonnull readonly align 1, %"core::fmt::Formatter"* align 8 dereferenceable(96)) unnamed_addr #1

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #7

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @memcmp(i8*, i8*, i64) unnamed_addr #5
