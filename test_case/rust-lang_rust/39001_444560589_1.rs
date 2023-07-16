llvm
; core::mem::uninitialized
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem13uninitialized17h4537c4eab9e44a67E([4096 x i8]* noalias nocapture sret dereferenceable(4096)) unnamed_addr #0 {
start:
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::zeroed
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem6zeroed17h4a86ff056d9543abE([4096 x i8]* noalias nocapture sret dereferenceable(4096)) unnamed_addr #0 {
start:
  %1 = bitcast [4096 x i8]* %0 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 4096, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::write_volatile
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr14write_volatile17h820044959ff57a02E([4096 x i8]* %dst, [4096 x i8]* noalias nocapture dereferenceable(4096) %src) unnamed_addr #0 {
start:
  %_5 = alloca [4096 x i8], align 1
  %0 = bitcast [4096 x i8]* %src to i8*
  %1 = bitcast [4096 x i8]* %_5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %0, i64 4096, i1 false)
  %2 = bitcast [4096 x i8]* %_5 to i8*
  %3 = bitcast [4096 x i8]* %dst to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %3, i8* align 1 %2, i64 4096, i1 true)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; test::slow
; Function Attrs: uwtable
define void @_ZN4test4slow17hb13ef2f0fb17248eE() unnamed_addr #1 {
start:
  %_7 = alloca [4096 x i8], align 1
  %buf = alloca [4096 x i8], align 1
; call core::mem::uninitialized
  call void @_ZN4core3mem13uninitialized17h4537c4eab9e44a67E([4096 x i8]* noalias nocapture sret dereferenceable(4096) %buf)
  br label %bb1

bb1:                                              ; preds = %start
; call core::mem::zeroed
  call void @_ZN4core3mem6zeroed17h4a86ff056d9543abE([4096 x i8]* noalias nocapture sret dereferenceable(4096) %_7)
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::ptr::write_volatile
  call void @_ZN4core3ptr14write_volatile17h820044959ff57a02E([4096 x i8]* %buf, [4096 x i8]* noalias nocapture dereferenceable(4096) %_7)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret void
}
