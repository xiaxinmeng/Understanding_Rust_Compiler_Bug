plain
    Checking cranelift-frontend v0.78.0
    Checking cranelift-native v0.78.0
    Checking cranelift-object v0.78.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 1 field
     |
     |
520  |                         | StatementKind::AscribeUserType(_, _)
     |                                                          ^  ^ expected 1 field, found 2
    ::: /checkout/compiler/rustc_middle/src/mir/mod.rs:1584:21
     |
     |
1584 |     AscribeUserType(Box<(Place<'tcx>, UserTypeProjection)>),
     |                     -------------------------------------- tuple variant has 1 field
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:37
