 llvm
; ModuleID = 'rust_out.0.rs'
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v64:64:64-v128:128:128-a:0:64-s0:64:64-f80:128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%str_slice = type { i8*, i64 }

@vtable895 = internal unnamed_addr constant { void (i8*)*, i64, i64, i64 (%str_slice*)* } { void (i8*)* @_ZN2i88drop.87717h403a8fc59605337eE, i64 16, i64 8, i64 (%str_slice*)* @_ZN3any5T.Any11get_type_id21h11409169050976825453E }
@str896 = internal constant [20 x i8] c"Das Ist N\C3\BCmberwang!"
@str898 = internal constant [6 x i8] c"<anon>"
@_ZN3foo10_FILE_LINE20h45522c5a40a253aeqaaE = internal constant { %str_slice, i32, [4 x i8] } { %str_slice { i8* getelementptr inbounds ([6 x i8], [6 x i8]* @str898, i32 0, i32 0), i64 6 }, i32 5, [4 x i8] undef }

; Function Attrs: cold noinline noreturn uwtable
define internal fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E() unnamed_addr #0 {
entry-block:
  %0 = tail call i8* @je_mallocx(i64 16, i32 0)
  %1 = icmp eq i8* %0, null
  br i1 %1, label %then-block-161-.i.i, label %"_ZN5boxed12Box$LT$T$GT$3new21h15424197560860407752E.exit"

then-block-161-.i.i:                              ; preds = %entry-block
  tail call void @_ZN3oom20hd47d86e88c71d41b96aE()
  unreachable

"_ZN5boxed12Box$LT$T$GT$3new21h15424197560860407752E.exit": ; preds = %entry-block
  %x.sroa.0.0..sroa_idx.i = bitcast i8* %0 to i8**
  store i8* getelementptr inbounds ([20 x i8], [20 x i8]* @str896, i64 0, i64 0), i8** %x.sroa.0.0..sroa_idx.i, align 8
  %x.sroa.4.0..sroa_idx2.i = getelementptr inbounds i8, i8* %0, i64 8
  %2 = bitcast i8* %x.sroa.4.0..sroa_idx2.i to i64*
  store i64 20, i64* %2, align 8
  tail call void @_ZN2rt6unwind18begin_unwind_inner20h30a8282e6074626cKPwE(i8* noalias nonnull %0, void (i8*)** nonnull getelementptr inbounds ({ void (i8*)*, i64, i64, i64 (%str_slice*)* }, { void (i8*)*, i64, i64, i64 (%str_slice*)* }* @vtable895, i64 0, i32 0), { %str_slice, i32 }* noalias readonly dereferenceable(24) bitcast ({ %str_slice, i32, [4 x i8] }* @_ZN3foo10_FILE_LINE20h45522c5a40a253aeqaaE to { %str_slice, i32 }*))
  unreachable
}

; Function Attrs: nounwind
declare void @llvm.lifetime.start(i64, i8* nocapture) unnamed_addr #1

; Function Attrs: cold noinline noreturn
declare void @_ZN2rt6unwind18begin_unwind_inner20h30a8282e6074626cKPwE(i8* noalias nonnull, void (i8*)** nonnull, { %str_slice, i32 }* noalias readonly dereferenceable(24)) unnamed_addr #2

; Function Attrs: nounwind
declare void @llvm.lifetime.end(i64, i8* nocapture) unnamed_addr #1

declare noalias i8* @je_mallocx(i64, i32) unnamed_addr #3

; Function Attrs: cold noinline noreturn
declare void @_ZN3oom20hd47d86e88c71d41b96aE() unnamed_addr #2

; Function Attrs: nounwind readnone
define internal void @_ZN2i88drop.87717h403a8fc59605337eE(i8* nocapture) unnamed_addr #4 {
entry-block:
  ret void
}

; Function Attrs: nounwind readnone uwtable
define internal i64 @_ZN3any5T.Any11get_type_id21h11409169050976825453E(%str_slice* noalias nocapture readonly dereferenceable(16)) unnamed_addr #5 {
entry-block:
  ret i64 8802261786977977801
}

; Function Attrs: uwtable
define internal void @_ZN4main20h84a5f37e14972344RaaE() unnamed_addr #6 {
entry-block:
  %value.i = alloca i32, align 4
  %0 = bitcast i32* %value.i to i8*
  call void @llvm.lifetime.start(i64 4, i8* %0) #7
  store i32 5, i32* %value.i, align 4
  call void asm sideeffect "", "r,~{dirflag},~{fpsr},~{flags}"(i32* nonnull %value.i) #7, !srcloc !0
  %1 = load i32, i32* %value.i, align 4
  call void @llvm.lifetime.end(i64 4, i8* %0) #7
  switch i32 %1, label %join [
    i32 1, label %match_case
    i32 48, label %match_case11
    i32 92, label %match_case12
    i32 23, label %match_case13
    i32 4, label %match_case14
    i32 70, label %match_case15
    i32 29, label %match_case16
    i32 36, label %match_case17
    i32 34, label %match_case18
    i32 58, label %match_case19
  ]

match_case:                                       ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case11:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case12:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case13:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case14:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case15:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case16:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case17:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case18:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

match_case19:                                     ; preds = %entry-block
  call fastcc void @_ZN2rt6unwind12begin_unwind21h14189931576037793481E()
  unreachable

join:                                             ; preds = %entry-block
  ret void
}

define i64 @main(i64, i8**) unnamed_addr #3 {
top:
  %2 = tail call i64 @_ZN2rt10lang_start20hdadb5fc58c5eeff334wE(i8* bitcast (void ()* @_ZN4main20h84a5f37e14972344RaaE to i8*), i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN2rt10lang_start20hdadb5fc58c5eeff334wE(i8*, i64, i8**) unnamed_addr #3

attributes #0 = { cold noinline noreturn uwtable "split-stack" }
attributes #1 = { nounwind "split-stack" }
attributes #2 = { cold noinline noreturn "split-stack" }
attributes #3 = { "split-stack" }
attributes #4 = { nounwind readnone "split-stack" }
attributes #5 = { nounwind readnone uwtable "split-stack" }
attributes #6 = { uwtable "split-stack" }
attributes #7 = { nounwind }

!0 = !{i32 4}
