llvm

%B8 = type { [8 x i8] }

@0 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 8

define void @_ZN4core3ptr14write_volatile17h0d514cc8a55c523bE(ptr %dst, i64 %0) unnamed_addr #0 !dbg !6 {
start:
  %1 = alloca i64, align 8
  %src = alloca %B8, align 8
  store i64 %0, ptr %1, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %src, ptr align 8 %1, i64 8, i1 false)
  ; the memcpy below is volatile
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %dst, ptr align 8 %src, i64 8, i1 true), !dbg !12
  ret void, !dbg !14
}

define void @_ZN7example10zeroize_b817hdf0aefd1fdb86108E(ptr align 8 %bytes) unnamed_addr #1 !dbg !15 {
start:
  %0 = alloca %B8, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 @0, i64 8, i1 false), !dbg !18
  %1 = load i64, ptr %0, align 8, !dbg !18
  call void @_ZN4core3ptr14write_volatile17h0d514cc8a55c523bE(ptr %bytes, i64 %1), !dbg !18
  ret void, !dbg !21
}

declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
