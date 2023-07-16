plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0425]: cannot find value `starts` in this scope
    --> src/builder.rs:1109:46
     |
1109 |                 let start = self.pointercast(starts[i], self.type_i8p());

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_codegen_gcc` (lib) due to previous error
Build completed unsuccessfully in 0:01:44
