plain
    Checking cranelift-frontend v0.83.0
    Checking cranelift-native v0.83.0
    Checking cranelift-object v0.83.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0599]: no method named `support_varargs` found for enum `rustc_target::spec::abi::Abi` in the current scope
   --> src/abi/mod.rs:490:28
    |
490 |             if !fn_sig.abi.support_varargs() {
    |                            ^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `supports_varargs`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:59
