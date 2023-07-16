 llvm
clean_custom_:                                    ; preds = %unwind_custom_
  call void @_ZN33cpu..ops..handlers..OpcodeHandler10drop.2140517h03d4f9d015a167dbE(%"cpu::ops::handlers::OpcodeHandler"* %60)
  %82 = bitcast %"cpu::ops::handlers::OpcodeHandler"* %60 to i8*
  call void @llvm.memset.p0i8.i64(i8* %82, i8 29, i64 40, i32 8, i1 false)
  br label %clean_ast_6925_

; ...

clean_custom_12:                                  ; preds = %unwind_custom_11
  call void @_ZN33cpu..ops..handlers..OpcodeHandler10drop.2140517h03d4f9d015a167dbE(%"cpu::ops::handlers::OpcodeHandler"* %67)
  %91 = bitcast %"cpu::ops::handlers::OpcodeHandler"* %67 to i8*
  call void @llvm.memset.p0i8.i64(i8* %91, i8 29, i64 40, i32 8, i1 false)
  call void @_ZN33cpu..ops..handlers..OpcodeHandler10drop.2140517h03d4f9d015a167dbE(%"cpu::ops::handlers::OpcodeHandler"* %60)
  %92 = bitcast %"cpu::ops::handlers::OpcodeHandler"* %60 to i8*
  call void @llvm.memset.p0i8.i64(i8* %92, i8 29, i64 40, i32 8, i1 false)
  br label %clean_ast_6925_
