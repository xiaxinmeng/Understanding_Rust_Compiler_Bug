
*** IR Dump Before MemorySanitizerPass on [module] ***
; Function Attrs: nonlazybind sanitize_memory uwtable
define i64 @penguin() unnamed_addr #0 !dbg !6 {
start:
  %dummy.dbg.spill = alloca {}, align 1
  call void @llvm.dbg.declare(metadata ptr %dummy.dbg.spill, metadata !13, metadata !DIExpression()), !dbg !17
  call void asm sideeffect "", "r,~{memory}"(ptr undef), !dbg !18, !srcloc !19
  br label %bb1, !dbg !18

bb1:                                              ; preds = %start
  ret i64 0, !dbg !20
}
*** IR Dump Before AnnotationRemarksPass on penguin ***
; Function Attrs: nonlazybind sanitize_memory uwtable
define i64 @penguin() unnamed_addr #0 !dbg !6 {
start:
  call void @llvm.donothing()
  %dummy.dbg.spill = alloca {}, align 1
  %0 = ptrtoint ptr %dummy.dbg.spill to i64
  %1 = xor i64 %0, 87960930222080
  %2 = inttoptr i64 %1 to ptr
  call void @llvm.memset.p0.i64(ptr align 1 %2, i8 -1, i64 0, i1 false)
  call void @llvm.dbg.declare(metadata ptr %dummy.dbg.spill, metadata !13, metadata !DIExpression()), !dbg !17
  call void @__msan_warning_noreturn() #5, !dbg !18
  call void asm sideeffect "", "r,~{memory}"(ptr undef), !dbg !18, !srcloc !19
  br label %bb1, !dbg !18

bb1:                                              ; preds = %start
  store i64 0, ptr @__msan_retval_tls, align 8, !dbg !20
  ret i64 0, !dbg !20
}
