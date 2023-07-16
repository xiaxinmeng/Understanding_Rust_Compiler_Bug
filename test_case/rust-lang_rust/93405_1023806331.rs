plain
    Checking cranelift-frontend v0.78.0
    Checking cranelift-native v0.78.0
    Checking cranelift-object v0.78.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0531]: cannot find unit struct, unit variant or constant `raw_eq` in module `sym`
     |
     |
1104 |         raw_eq, <T>(v lhs_ref, v rhs_ref) {
     |         ^^^^^^ not found in `sym`
For more information about this error, try `rustc --explain E0531`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:05
