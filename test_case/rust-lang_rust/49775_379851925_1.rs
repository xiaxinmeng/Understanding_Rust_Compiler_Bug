ll
@FOO = internal unnamed_addr global <{ [4 x i8] }> zeroinitializer, align 4

define void @main() {
  %a = load atomic i32, i32* bitcast (<{ [4 x i8] }>* @FOO to i32*) seq_cst, align 4
  ret void
}
