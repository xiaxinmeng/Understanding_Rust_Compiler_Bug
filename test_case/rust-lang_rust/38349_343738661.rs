diff
 ; test::id_result
 ; Function Attrs: uwtable
-define void @_ZN4test9id_result17h0e83cb02e8136307E(%"core::result::Result<u64, i64>"* noalias nocapture sret dereferenceable(16), %"core::result::Result<u64, i64>"* noalias nocapture dereferenceable(16)) unnamed_addr #0 {
+define void @_ZN4test9id_result17ha0e8372a885d01bfE(%"core::result::Result<u64, i64>"* noalias nocapture sret dereferenceable(16), %"core::result::Result<u64, i64>"* noalias nocapture dereferenceable(16) %a) unnamed_addr #0 {
 start:
-  %a = alloca %"core::result::Result<u64, i64>"
-  %2 = bitcast %"core::result::Result<u64, i64>"* %1 to i8*
-  %3 = bitcast %"core::result::Result<u64, i64>"* %a to i8*
-  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %3, i8* %2, i64 16, i32 8, i1 false)
-  %4 = getelementptr inbounds %"core::result::Result<u64, i64>", %"core::result::Result<u64, i64>"* %a, i32 0, i32 0
-  %5 = load i64, i64* %4, !range !0
-  switch i64 %5, label %bb2 [
+  %1 = getelementptr inbounds %"core::result::Result<u64, i64>", %"core::result::Result<u64, i64>"* %a, i32 0, i32 0
+  %2 = load i64, i64* %1, !range !0
+  switch i64 %2, label %bb2 [
     i64 0, label %bb1
   ]
(snip)
