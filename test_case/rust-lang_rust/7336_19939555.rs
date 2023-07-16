 llvm
; ModuleID = 'issue7336.rc'
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v64:64:64-v128:128:128-a0:0:64-s0:64:64-f80:128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%tydesc = type { i64, i64, void ({}*, %tydesc**, i8*)*, void ({}*, %tydesc**, i8*)*, void ({}*, %tydesc**, i8*)*, void ({}*, %tydesc**, i8*)*, i8*, i8* }

@_rust_crate_map_toplevel = global { i32, i8*, i64, [2 x i64] } { i32 1, i8* bitcast (void ({ i64, %tydesc*, i8*, i8*, i8 }*)* @_ZN7cleanup10annihilate16_82984335c95fdd56_07preE to i8*), i64 ptrtoint ([1 x { i64, i64 }]* @_rust_mod_map to i64), [2 x i64] [i64 ptrtoint (i64* @_rust_crate_map_std_0.7-pre_6c65cf4b443341b1 to i64), i64 0] }
@_rust_crate_map_std_0.7-pre_6c65cf4b443341b1 = external global i64
@_rust_mod_map = internal global [1 x { i64, i64 }] zeroinitializer
@rust_abi_version = constant i64 1

define void @_rust_main({ i64, %tydesc*, i8*, i8*, i8 }* nocapture) #0 {
static_allocas:
  ret void
}

define i64 @main(i64, i8**) {
top:
  %2 = tail call i64 @_ZN8unstable4lang5start17_76d6c774aa357c7a6_07preE({ i64, %tydesc*, i8*, i8*, i8 }* null, i8* bitcast (void ({ i64, %tydesc*, i8*, i8*, i8 }*)* @_rust_main to i8*), i64 %0, i8** %1, i8* bitcast ({ i32, i8*, i64, [2 x i64] }* @_rust_crate_map_toplevel to i8*))
  ret i64 %2
}

declare i64 @_ZN8unstable4lang5start17_76d6c774aa357c7a6_07preE({ i64, %tydesc*, i8*, i8*, i8 }*, i8*, i64, i8**, i8*)

declare void @_ZN7cleanup10annihilate16_82984335c95fdd56_07preE({ i64, %tydesc*, i8*, i8*, i8 }*)

declare void @llvm.lifetime.start(i64, i8* nocapture) #1

declare void @llvm.lifetime.end(i64, i8* nocapture) #1

attributes #0 = { readnone }
attributes #1 = { nounwind }
