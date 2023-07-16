llvm
; ModuleID = 'a.7rcbfp3g-cgu.0'
source_filename = "a.7rcbfp3g-cgu.0"
target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128"
target triple = "i686-unknown-linux-gnu"

@vtable.0 = private unnamed_addr constant { void (i8**)*, i32, i32, i32 (i8**)*, i32 (i8**)*, i32 (i8**)* } { void (i8**)* @_ZN4core3ptr13drop_in_place17hc179b6618de53956E, i32 4, i32 4, i32 (i8**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcdc8ef052ef043e4E", i32 (i8**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcdc8ef052ef043e4E", i32 (i8**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1aff92f1e37de118E" }, align 4
@alloc12 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"Overflow" }>, align 1
@alloc13 = private unnamed_addr constant <{ [7 x i8] }> <{ [7 x i8] c"Timeout" }>, align 1
@alloc14 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"Busy" }>, align 1
@alloc15 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"NotFound" }>, align 1

; std::rt::lang_start
; Function Attrs: minsize nounwind nonlazybind optsize
define hidden i32 @_ZN3std2rt10lang_start17h5d438b8c073bcdeeE(void ()* nonnull %main, i32 %argc, i8** %argv) unnamed_addr #0 {
start:
  %_7 = alloca i8*, align 4
  %0 = bitcast i8** %_7 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %0)
  %1 = bitcast i8** %_7 to void ()**
  store void ()* %main, void ()** %1, align 4
  %_4.0 = bitcast i8** %_7 to {}*
; call std::rt::lang_start_internal
  %2 = call i32 @_ZN3std2rt19lang_start_internal17h0cdc5e5d4c9570d9E({}* nonnull align 1 %_4.0, [3 x i32]* noalias readonly align 4 dereferenceable(12) bitcast ({ void (i8**)*, i32, i32, i32 (i8**)*, i32 (i8**)*, i32 (i8**)* }* @vtable.0 to [3 x i32]*), i32 %argc, i8** %argv)
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %0)
  ret i32 %2
}

; std::rt::lang_start::{{closure}}
; Function Attrs: minsize nounwind nonlazybind optsize
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcdc8ef052ef043e4E"(i8** noalias nocapture readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 {
start:
  %0 = bitcast i8** %_1 to void ()**
  %_3 = load void ()*, void ()** %0, align 4, !nonnull !3
  tail call void %_3() #6
  ret i32 0
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: minsize nounwind nonlazybind optsize
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1aff92f1e37de118E"(i8** nocapture readonly %_1) unnamed_addr #0 {
start:
  %0 = bitcast i8** %_1 to void ()**
  %1 = load void ()*, void ()** %0, align 4, !nonnull !3
  tail call void %1() #6, !noalias !4
  ret i32 0
}

; core::ptr::drop_in_place
; Function Attrs: minsize norecurse nounwind nonlazybind optsize readnone
define internal void @_ZN4core3ptr13drop_in_place17hc179b6618de53956E(i8** nocapture %_1) unnamed_addr #1 {
start:
  ret void
}

; a::Error::strerror
; Function Attrs: minsize noinline norecurse nounwind nonlazybind optsize readnone
define internal { [0 x i8]*, i32 } @_ZN1a5Error8strerror17h116d2107cc85b4f7E(i8 %0) unnamed_addr #2 {
start:
  switch i8 %0, label %bb2 [
    i8 0, label %bb3
    i8 1, label %bb4
    i8 2, label %bb5
    i8 3, label %bb6
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  br label %bb6

bb4:                                              ; preds = %start
  br label %bb6

bb5:                                              ; preds = %start
  br label %bb6

bb6:                                              ; preds = %start, %bb3, %bb4, %bb5
  %.sroa.0.0 = phi [0 x i8]* [ bitcast (<{ [7 x i8] }>* @alloc13 to [0 x i8]*), %bb5 ], [ bitcast (<{ [4 x i8] }>* @alloc14 to [0 x i8]*), %bb4 ], [ bitcast (<{ [8 x i8] }>* @alloc15 to [0 x i8]*), %bb3 ], [ bitcast (<{ [8 x i8] }>* @alloc12 to [0 x i8]*), %start ]
  %.sroa.5.0 = phi i32 [ 7, %bb5 ], [ 4, %bb4 ], [ 8, %bb3 ], [ 8, %start ]
  %1 = insertvalue { [0 x i8]*, i32 } undef, [0 x i8]* %.sroa.0.0, 0
  %2 = insertvalue { [0 x i8]*, i32 } %1, i32 %.sroa.5.0, 1
  ret { [0 x i8]*, i32 } %2
}

; a::main
; Function Attrs: minsize noreturn nounwind nonlazybind optsize
define internal void @_ZN1a4main17h5a7c080779719c70E() unnamed_addr #3 {
start:
  %dummy.i = alloca { [0 x i8]*, i32 } (i8)*, align 4
  %0 = bitcast { [0 x i8]*, i32 } (i8)** %dummy.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %0)
  store { [0 x i8]*, i32 } (i8)* @_ZN1a5Error8strerror17h116d2107cc85b4f7E, { [0 x i8]*, i32 } (i8)** %dummy.i, align 4
  call void asm sideeffect "", "r,~{dirflag},~{fpsr},~{flags}"({ [0 x i8]*, i32 } (i8)** nonnull %dummy.i) #6, !srcloc !7
  %1 = load { [0 x i8]*, i32 } (i8)*, { [0 x i8]*, i32 } (i8)** %dummy.i, align 4, !nonnull !3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %0)
  %2 = call { [0 x i8]*, i32 } %1(i8 2) #6
  %_6.1 = extractvalue { [0 x i8]*, i32 } %2, 1
; call std::process::exit
  call void @_ZN3std7process4exit17h0bec05e35ebb7815E(i32 %_6.1)
  unreachable
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #4

; std::rt::lang_start_internal
; Function Attrs: minsize nounwind nonlazybind optsize
declare i32 @_ZN3std2rt19lang_start_internal17h0cdc5e5d4c9570d9E({}* nonnull align 1, [3 x i32]* noalias readonly align 4 dereferenceable(12), i32, i8**) unnamed_addr #0

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #4

; std::process::exit
; Function Attrs: minsize noreturn nounwind nonlazybind optsize
declare void @_ZN3std7process4exit17h0bec05e35ebb7815E(i32) unnamed_addr #3

; Function Attrs: minsize nounwind nonlazybind optsize
define i32 @main(i32 %0, i8** %1) unnamed_addr #5 {
top:
; call std::rt::lang_start
  %2 = tail call i32 @_ZN3std2rt10lang_start17h5d438b8c073bcdeeE(void ()* @_ZN1a4main17h5a7c080779719c70E, i32 %0, i8** %1)
  ret i32 %2
}

attributes #0 = { minsize nounwind nonlazybind optsize "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }
attributes #1 = { minsize norecurse nounwind nonlazybind optsize readnone "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }
attributes #2 = { minsize noinline norecurse nounwind nonlazybind optsize readnone "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }
attributes #3 = { minsize noreturn nounwind nonlazybind optsize "probe-stack"="__rust_probestack" "target-cpu"="pentium4" }
attributes #4 = { argmemonly nounwind willreturn }
attributes #5 = { minsize nounwind nonlazybind optsize "target-cpu"="pentium4" }
attributes #6 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcdc8ef052ef043e4E: %_1"}
!6 = distinct !{!6, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hcdc8ef052ef043e4E"}
!7 = !{i32 2989080}
