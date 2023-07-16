 llvm
define internal void @_ZN3foo20h9f580e219f1e2e10paaE(%"struct.BigStruct<[]>"* noalias nocapture sret dereferenceable(24), %"struct.BigStruct<[]>"* noalias dereferenceable(24)) unnamed_addr #0 {
entry-block:
  %x = alloca %"struct.BigStruct<[]>"*
  store %"struct.BigStruct<[]>"* %1, %"struct.BigStruct<[]>"** %x
  %2 = load %"struct.BigStruct<[]>"** %x
  %3 = bitcast %"struct.BigStruct<[]>"* %2 to i8*
  %4 = bitcast %"struct.BigStruct<[]>"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %3, i64 24, i32 8, i1 false)
  br label %clean_custom_

return:                                           ; preds = %clean_custom_
  ret void

clean_custom_:                                    ; preds = %entry-block
  call void @"_ZN20Box$LT$BigStruct$GT$14glue_drop.108817h69e26cab4c19e281E"(%"struct.BigStruct<[]>"** %x)
  br label %return
}
