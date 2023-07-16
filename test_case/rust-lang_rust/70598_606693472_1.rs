llvm
target triple = "x86_64-unknown-linux-gnu"

@_ZN10playground1A17hfc698d88543494bfE = internal thread_local constant <{ [1 x i8] }> zeroinitializer, align 1, !dbg !0

; playground::get_a_ref
; Function Attrs: nonlazybind uwtable
define i8* @_ZN10playground9get_a_ref17h42f05c46255e0587E() unnamed_addr #0 !dbg !11 {
start:
  ret i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @_ZN10playground1A17hfc698d88543494bfE, i32 0, i32 0, i32 0), !dbg !15
}
