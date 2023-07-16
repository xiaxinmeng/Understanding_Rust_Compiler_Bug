plain
    Checking cranelift-native v0.81.0
    Checking cranelift-frontend v0.81.0
    Checking cranelift-object v0.81.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `UninitVariant(_, _)` not covered
     |
     |
489  |             match to_place_and_rval.1 {
     |                   ^^^^^^^^^^^^^^^^^^^ pattern `UninitVariant(_, _)` not covered
    ::: /checkout/compiler/rustc_middle/src/mir/mod.rs:2271:5
     |
     |
2271 |     UninitVariant(Ty<'tcx>, VariantIdx),
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `rustc_middle::mir::Rvalue`

