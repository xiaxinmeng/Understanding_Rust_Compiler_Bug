plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/base.rs:720:32
     |
720  |                 Rvalue::Repeat(ref operand, times) => {
     |
    ::: /checkout/compiler/rustc_middle/src/mir/syntax.rs:1013:12
     |
     |
1013 |     Repeat(Operand<'tcx>, ty::Const<'tcx>, bool),
     |
help: use `_` to explicitly ignore each field
     |
     |
720  |                 Rvalue::Repeat(ref operand, times, _) => {

For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:45
