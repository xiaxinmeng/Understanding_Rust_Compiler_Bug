llvm
; ModuleID = 'c.ll'
source_filename = "b.7rcbfp3g-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@anon.7406e8c0c880c9bceef10e4e4055b7c3.1 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"ab\0A\00" }>, align 1

; Function Attrs: inlinehint nonlazybind uwtable
define internal { [0 x i8]*, i64 } @from_raw_parts(i8* %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast i8* %data to [0 x i8]*
  %1 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %0, 0
  %2 = insertvalue { [0 x i8]*, i64 } %1, i64 %len, 1
  ret { [0 x i8]*, i64 } %2
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @slice_len([0 x i8]* noalias nonnull readonly align 1 %data, i64 %len) unnamed_addr #0 {
start:
  ret i64 %len
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal i8* @as_ptr([0 x i8]* noalias nonnull readonly align 1 %data, i64 %len) unnamed_addr #0 {
start:
  %0 = bitcast [0 x i8]* %data to i8*
  ret i8* %0
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal { [0 x i8]*, i64 } @deref({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i64 0, i32 0
  %_2 = load i8*, i8** %0, align 8
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i64 0, i32 1
  %_3 = load i64, i64* %1, align 8
  %2 = call { [0 x i8]*, i64 } @from_raw_parts(i8* %_2, i64 %_3)
  ret { [0 x i8]*, i64 } %2
}

; Function Attrs: noinline norecurse nounwind nonlazybind optnone readnone uwtable
define { i8*, i64 } @crate() unnamed_addr #2 {
  ret { i8*, i64 } { i8* getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @anon.7406e8c0c880c9bceef10e4e4055b7c3.1, i64 0, i32 0, i64 0), i64 3 }
}

; Function Attrs: nonlazybind
declare dso_local i64 @write(i32, i8* nocapture readonly, i64) local_unnamed_addr #3

; Function Attrs: nonlazybind
define i32 @main(i32 %argc, i8** %argv) unnamed_addr #3 {
start:
  %ptr = alloca { i8*, i64 }, align 8
  %0 = call { i8*, i64 } @crate()
  store { i8*, i64 } %0, { i8*, i64 }* %ptr, align 8
  %1 = call { [0 x i8]*, i64 } @deref({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %ptr)
  %_1.0 = extractvalue { [0 x i8]*, i64 } %1, 0
  %_1.1 = extractvalue { [0 x i8]*, i64 } %1, 1
  %2 = call i8* @as_ptr([0 x i8]* noalias nonnull readonly align 1 %_1.0, i64 %_1.1)
  %3 = call { [0 x i8]*, i64 } @deref({ i8*, i64 }* noalias readonly align 8 dereferenceable(16) %ptr)
  %_3.0 = extractvalue { [0 x i8]*, i64 } %3, 0
  %_3.1 = extractvalue { [0 x i8]*, i64 } %3, 1
  %4 = call i64 @slice_len([0 x i8]* noalias nonnull readonly align 1 %_3.0, i64 %_3.1)
  %5 = call { [0 x i8]*, i64 } @from_raw_parts(i8* %2, i64 %4)
  %_5.0 = extractvalue { [0 x i8]*, i64 } %5, 0
  %_5.1 = extractvalue { [0 x i8]*, i64 } %5, 1
  %6 = bitcast [0 x i8]* %_5.0 to i8*
  %7 = call i64 @write(i32 1, i8* nonnull %6, i64 %_5.1)
  ret i32 0
}

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { noinline norecurse nounwind nonlazybind optnone readnone uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { nonlazybind "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
