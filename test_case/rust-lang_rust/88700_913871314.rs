plain
    Checking cranelift-frontend v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-native v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `InitBox(_, _)` not covered
     |
     |
459  |             match to_place_and_rval.1 {
     |                   ^^^^^^^^^^^^^^^^^^^ pattern `InitBox(_, _)` not covered
    ::: /checkout/compiler/rustc_middle/src/mir/mod.rs:2174:5
     |
     |
2174 |     InitBox(Operand<'tcx>, Ty<'tcx>),
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `rustc_middle::mir::Rvalue`

