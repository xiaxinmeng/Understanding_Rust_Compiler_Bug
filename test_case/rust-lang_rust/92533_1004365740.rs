plain
    Checking cranelift-native v0.78.0
    Checking cranelift-frontend v0.78.0
    Checking cranelift-object v0.78.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0615]: attempted to take value of method `ident` on type `&rustc_middle::ty::FieldDef`
   --> src/debuginfo/mod.rs:177:58
    |
177 |                         AttributeValue::String(field_def.ident.as_str().to_string().into_bytes()),
    |
help: use parentheses to call the method
    |
    |
177 |                         AttributeValue::String(field_def.ident(_).as_str().to_string().into_bytes()),

For more information about this error, try `rustc --explain E0615`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:14
