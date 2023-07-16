 diff
--- 0.11.ll 2014-07-18 23:38:18.000000000 +0200
+++ master.ll   2014-07-18 23:37:50.000000000 +0200
@@ -1,20 +1,21 @@
 %"enum.Enum<[]>" = type { i8, [0 x i8], [1 x i8] }

 ; Function Attrs: inlinehint
-define internal %"enum.Enum<[]>" @_ZN8Variant120h2026663588c21502jaa4v0.0E(i1 zeroext) unnamed_addr #0 {
+define internal %"enum.Enum<[]>" @_ZN8Variant120he59273a13bfc9b2fjaaE(i1 zeroext) unnamed_addr #0 {
 entry-block:
   %__make_return_pointer = alloca %"enum.Enum<[]>"
   %1 = getelementptr inbounds %"enum.Enum<[]>"* %__make_return_pointer, i32 0, i32 0
   store i8 0, i8* %1
-  %2 = bitcast %"enum.Enum<[]>"* %__make_return_pointer to { i8, i1 }*
-  %3 = getelementptr inbounds { i8, i1 }* %2, i32 0, i32 1
-  store i1 %0, i1* %3
-  %4 = load %"enum.Enum<[]>"* %__make_return_pointer
-  ret %"enum.Enum<[]>" %4
+  %2 = bitcast %"enum.Enum<[]>"* %__make_return_pointer to { i8, i8 }*
+  %3 = getelementptr inbounds { i8, i8 }* %2, i32 0, i32 1
+  %4 = zext i1 %0 to i8
+  store i8 %4, i8* %3
+  %5 = load %"enum.Enum<[]>"* %__make_return_pointer
+  ret %"enum.Enum<[]>" %5
 }

 ; Function Attrs: inlinehint
-define internal %"enum.Enum<[]>" @_ZN8Variant220h4237def28a143758naa4v0.0E(i8) unnamed_addr #0 {
+define internal %"enum.Enum<[]>" @_ZN8Variant220he630b7b489d87862naaE(i8) unnamed_addr #0 {
 entry-block:
   %__make_return_pointer = alloca %"enum.Enum<[]>"
   %1 = getelementptr inbounds %"enum.Enum<[]>"* %__make_return_pointer, i32 0, i32 0
@@ -27,7 +28,7 @@
 }

 ; Function Attrs: uwtable
-define internal void @_ZN4main20h22638fb321d781d6raa4v0.0E() unnamed_addr #1 {
+define internal void @_ZN4main20hd74a607d35282c19raaE() unnamed_addr #1 {
 entry-block:
   %0 = alloca i64
   %1 = alloca %"enum.Enum<[]>"
@@ -35,7 +36,7 @@
   %match = alloca %"enum.Enum<[]>"
   store i8 2, i8* %2
   %3 = load i8* %2
-  %4 = call %"enum.Enum<[]>" @_ZN8Variant220h4237def28a143758naa4v0.0E(i8 %3)
+  %4 = call %"enum.Enum<[]>" @_ZN8Variant220he630b7b489d87862naaE(i8 %3)
   store %"enum.Enum<[]>" %4, %"enum.Enum<[]>"* %1
   %5 = load %"enum.Enum<[]>"* %1
   store %"enum.Enum<[]>" %5, %"enum.Enum<[]>"* %match
@@ -75,11 +76,12 @@
   ]

 match_case:                                       ; preds = %entry-block
-  %11 = bitcast %"enum.Enum<[]>"* %match to { i8, i1 }*
-  %12 = getelementptr inbounds { i8, i1 }* %11, i32 0, i32 1
-  %13 = load i1* %12
-  %14 = icmp eq i1 %13, true
-  br i1 %14, label %match_case6, label %compare_next
+  %11 = bitcast %"enum.Enum<[]>"* %match to { i8, i8 }*
+  %12 = getelementptr inbounds { i8, i8 }* %11, i32 0, i32 1
+  %13 = load i8* %12, !range !0
+  %14 = trunc i8 %13 to i1
+  %15 = icmp eq i1 %14, true
+  br i1 %15, label %match_case6, label %compare_next

 match_else5:                                      ; preds = %compare_next
   br label %case_body1
@@ -100,20 +102,20 @@
   br label %case_body3

 join:                                             ; preds = %case_body4, %case_body3, %case_body2, %case_body1, %case_body
-  %15 = load i64* %0
-  call void @_ZN2os15set_exit_status20ha1905d2c01844302Ivg7v0.11.0E(i64 %15)
+  %16 = load i64* %0
+  call void @_ZN2os15set_exit_status20h6ce0af8bbd777aa29zgE(i64 %16)
   ret void
 }

 define i64 @main(i64, i8**) unnamed_addr #2 {
 top:
-  %2 = call i64 @_ZN10lang_start20hfd2bc187bf8dda06Fme7v0.11.0E(i8* bitcast (void ()* @_ZN4main20h22638fb321d781d6raa4v0.0E to i8*), i64 %0, i8** %1)
+  %2 = call i64 @_ZN10lang_start20hea9a1813d80388d7WjeE(i8* bitcast (void ()* @_ZN4main20hd74a607d35282c19raaE to i8*), i64 %0, i8** %1)
   ret i64 %2
 }

-declare i64 @_ZN10lang_start20hfd2bc187bf8dda06Fme7v0.11.0E(i8*, i64, i8**) unnamed_addr #2
+declare i64 @_ZN10lang_start20hea9a1813d80388d7WjeE(i8*, i64, i8**) unnamed_addr #2

-declare void @_ZN2os15set_exit_status20ha1905d2c01844302Ivg7v0.11.0E(i64) unnamed_addr #2
+declare void @_ZN2os15set_exit_status20h6ce0af8bbd777aa29zgE(i64) unnamed_addr #2

 attributes #0 = { inlinehint "split-stack" }
 attributes #1 = { uwtable "split-stack" }
