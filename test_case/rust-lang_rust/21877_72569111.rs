 diff
--- before.ll   2015-02-03 01:33:13.730424913 +0100
+++ after.ll    2015-02-03 01:33:07.814471400 +0100
@@ -1,65 +1,37 @@
-define internal void @_ZN3mem4swap21h11318933936959990132E(i32* noalias dereferenceable(4), i32* noalias dereferenceable(4)) unnamed_addr #3 {
+define internal void @_ZN3mem4swap21h15317848707200848053E(i32* noalias dereferenceable(4), i32* noalias dereferenceable(4)) unnamed_addr #3 {
 entry-block:
   %x = alloca i32*
   %y = alloca i32*
   %t = alloca i32
-  %auto_deref = alloca i32*
-  %auto_deref1 = alloca i32*
-  %auto_deref2 = alloca i32*
-  %auto_deref3 = alloca i32*
   %2 = bitcast i32** %x to i8*
   call void @llvm.lifetime.start(i64 8, i8* %2)
   store i32* %0, i32** %x
   %3 = bitcast i32** %y to i8*
   call void @llvm.lifetime.start(i64 8, i8* %3)
   store i32* %1, i32** %y
   %4 = bitcast i32* %t to i8*
   call void @llvm.lifetime.start(i64 4, i8* %4)
-  %5 = call i32 @_ZN3mem13uninitialized20h8430628305090271874E()
+  %5 = call i32 @_ZN3mem13uninitialized20h2747246305834344603E()
   store i32 %5, i32* %t
-  %6 = bitcast i32** %auto_deref to i8*
-  call void @llvm.lifetime.start(i64 8, i8* %6)
-  store i32* %t, i32** %auto_deref
-  %7 = load i32** %auto_deref
-  %8 = load i32** %x
-  %9 = bitcast i32** %auto_deref1 to i8*
-  call void @llvm.lifetime.start(i64 8, i8* %9)
-  store i32* %8, i32** %auto_deref1
-  %10 = load i32** %auto_deref1
-  %11 = bitcast i32* %7 to i8*
+  %6 = load i32** %x
+  %7 = bitcast i32* %t to i8*
+  %8 = bitcast i32* %6 to i8*
+  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %7, i8* %8, i64 4, i32 4, i1 false)
+  %9 = load i32** %x
+  %10 = load i32** %y
+  %11 = bitcast i32* %9 to i8*
   %12 = bitcast i32* %10 to i8*
   call void @llvm.memcpy.p0i8.p0i8.i64(i8* %11, i8* %12, i64 4, i32 4, i1 false)
-  %13 = bitcast i32** %auto_deref1 to i8*
-  call void @llvm.lifetime.end(i64 8, i8* %13)
-  %14 = bitcast i32** %auto_deref to i8*
-  call void @llvm.lifetime.end(i64 8, i8* %14)
-  %15 = load i32** %x
-  %16 = load i32** %y
-  %17 = bitcast i32** %auto_deref2 to i8*
-  call void @llvm.lifetime.start(i64 8, i8* %17)
-  store i32* %16, i32** %auto_deref2
-  %18 = load i32** %auto_deref2
-  %19 = bitcast i32* %15 to i8*
-  %20 = bitcast i32* %18 to i8*
-  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %19, i8* %20, i64 4, i32 4, i1 false)
-  %21 = bitcast i32** %auto_deref2 to i8*
-  call void @llvm.lifetime.end(i64 8, i8* %21)
-  %22 = load i32** %y
-  %23 = bitcast i32** %auto_deref3 to i8*
-  call void @llvm.lifetime.start(i64 8, i8* %23)
-  store i32* %t, i32** %auto_deref3
-  %24 = load i32** %auto_deref3
-  %25 = bitcast i32* %22 to i8*
-  %26 = bitcast i32* %24 to i8*
-  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %25, i8* %26, i64 4, i32 4, i1 false)
-  %27 = bitcast i32** %auto_deref3 to i8*
-  call void @llvm.lifetime.end(i64 8, i8* %27)
-  %28 = load i32* %t
-  %29 = bitcast i32* %t to i8*
-  call void @llvm.lifetime.end(i64 4, i8* %29)
-  %30 = bitcast i32** %y to i8*
-  call void @llvm.lifetime.end(i64 8, i8* %30)
-  %31 = bitcast i32** %x to i8*
-  call void @llvm.lifetime.end(i64 8, i8* %31)
+  %13 = load i32** %y
+  %14 = bitcast i32* %13 to i8*
+  %15 = bitcast i32* %t to i8*
+  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %14, i8* %15, i64 4, i32 4, i1 false)
+  %16 = load i32* %t
+  %17 = bitcast i32* %t to i8*
+  call void @llvm.lifetime.end(i64 4, i8* %17)
+  %18 = bitcast i32** %y to i8*
+  call void @llvm.lifetime.end(i64 8, i8* %18)
+  %19 = bitcast i32** %x to i8*
+  call void @llvm.lifetime.end(i64 8, i8* %19)
   ret void
 }
