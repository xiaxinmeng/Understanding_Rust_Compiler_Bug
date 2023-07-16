diff
diff --git a/llvm/lib/IR/Mangler.cpp b/llvm/lib/IR/Mangler.cpp
index bbdde586e6e0..0a61174c638c 100644
--- a/llvm/lib/IR/Mangler.cpp
+++ b/llvm/lib/IR/Mangler.cpp
@@ -99,6 +99,11 @@ static void addByteCountSuffix(raw_ostream &OS, const Function *F,
   const unsigned PtrSize = DL.getPointerSize();

   for (const Argument &A : F->args()) {
+    // For the purposes of the byte count suffix, structs returned by pointer
+    // do not count as function arguments.
+    if (A.hasStructRetAttr())
+      continue;
+
     // 'Dereference' type in case of byval or inalloca parameter attribute.
     uint64_t AllocSize = A.hasPassPointeeByValueCopyAttr() ?
       A.getPassPointeeByValueCopySize(DL) :
diff --git a/llvm/test/CodeGen/X86/stdcall.ll b/llvm/test/CodeGen/X86/stdcall.ll
index 3cefe14fe0d5..25107c5d93f2 100644
--- a/llvm/test/CodeGen/X86/stdcall.ll
+++ b/llvm/test/CodeGen/X86/stdcall.ll
@@ -18,6 +18,21 @@ entry:
   ret i32 %a
 }

+%struct.large_type = type { i64, i64, i64 }
+
+define x86_stdcallcc void @ReturnLargeType(%struct.large_type* noalias nocapture sret(%struct.large_type) align 8 %agg.result) {
+; CHECK: ReturnLargeType@0:
+; CHECK: retl
+entry:
+  %a = getelementptr inbounds %struct.large_type, %struct.large_type* %agg.result, i32 0, i32 0
+  store i64 123, i64* %a, align 8
+  %b = getelementptr inbounds %struct.large_type, %struct.large_type* %agg.result, i32 0, i32 1
+  store i64 456, i64* %b, align 8
+  %c = getelementptr inbounds %struct.large_type, %struct.large_type* %agg.result, i32 0, i32 2
+  store i64 789, i64* %c, align 8
+  ret void
+}
+
 @B = global %0 { void (...)* bitcast (void ()* @MyFunc to void (...)*) }, align 4
 ; CHECK: _B:
 ; CHECK: .long _MyFunc@0
diff --git a/llvm/test/CodeGen/X86/vectorcall.ll b/llvm/test/CodeGen/X86/vectorcall.ll
index d3d44f6bdd7a..b8d53eaf0bba 100644
--- a/llvm/test/CodeGen/X86/vectorcall.ll
+++ b/llvm/test/CodeGen/X86/vectorcall.ll
@@ -172,8 +172,7 @@ declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture r
 declare void @llvm.memcpy.p0i8.p0i8.i32(i8* nocapture writeonly, i8* nocapture readonly, i32, i1)

 define x86_vectorcallcc void @test_mixed_7(%struct.HVA5* noalias sret(%struct.HVA5) %agg.result) {
-; X86-LABEL: test_mixed_7@@4
-; X64-LABEL: test_mixed_7@@8
+; CHECK-LABEL: test_mixed_7@@0
 ; X64:         mov{{[ql]}}     %rcx, %rax
 ; CHECK:       movaps  %xmm{{[0-9]}}, 64(%{{rcx|eax}})
 ; CHECK:       movaps  %xmm{{[0-9]}}, 48(%{{rcx|eax}})
