 llvm
define internal fastcc void @_ZN8function19hf4c7ff781ac735e1an4v0.0E(%"enum.Option<proc:Send()>"* nocapture, { void ({ i64, %tydesc*, i8*, i8*, i8 }*)*, { i64, %tydesc*, i8*, i8*, i8 }* }* noalias nocapture readonly) unnamed_addr #1 {
normal-return:
  %.sroa.2 = alloca [15 x i8], align 1
  %_breaker = alloca { void ({ i64, %tydesc*, i8*, i8*, i8 }*)*, { i64, %tydesc*, i8*, i8*, i8 }* }, align 8
  %arg = alloca { void ({ i64, %tydesc*, i8*, i8*, i8 }*)*, { i64, %tydesc*, i8*, i8*, i8 }* }, align 8
  %f.sroa.0.0.idx = getelementptr inbounds { void ({ i64, %tydesc*, i8*, i8*, i8 }*)*, { i64, %tydesc*, i8*, i8*, i8 }* }* %1, i64 0, i32 0
  %f.sroa.0.0.copyload = load void ({ i64, %tydesc*, i8*, i8*, i8 }*)** %f.sroa.0.0.idx, align 8
  %f.sroa.5.0.idx19 = getelementptr inbounds { void ({ i64, %tydesc*, i8*, i8*, i8 }*)*, { i64, %tydesc*, i8*, i8*, i8 }* }* %1, i64 0, i32 1
  %f.sroa.5.0.copyload = load { i64, %tydesc*, i8*, i8*, i8 }** %f.sroa.5.0.idx19, align 8
  %.sroa.2.1.idx10 = getelementptr inbounds [15 x i8]* %.sroa.2, i64 0, i64 0

  ; Loads from slot occur here
  %tmp.sroa.0.0.idx6.i.i = getelementptr inbounds %"enum.Option<proc:Send()>"* %0, i64 0, i32 0

  ; Discriminator for slot
  %tmp.sroa.0.0.copyload7.i.i = load i8* %tmp.sroa.0.0.idx6.i.i, align 8
  %tmp.sroa.6.0.idx12.i.i = getelementptr inbounds %"enum.Option<proc:Send()>"* %0, i64 0, i32 1, i64 0
  %tmp.sroa.616.0.idx17.i.i = getelementptr inbounds %"enum.Option<proc:Send()>"* %0, i64 0, i32 2, i64 1
  %tmp.sroa.616.0.cast18.i.i = bitcast i64* %tmp.sroa.616.0.idx17.i.i to { i64, %tydesc*, i8*, i8*, i8 }**

  ; Env pointer for slot    
  %tmp.sroa.616.0.copyload19.i.i = load { i64, %tydesc*, i8*, i8*, i8 }** %tmp.sroa.616.0.cast18.i.i, align 8
