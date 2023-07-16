plain
    Checking cranelift-frontend v0.73.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f)
    Checking cranelift-object v0.73.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f)
    Checking cranelift-jit v0.73.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#45bee40f)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0425]: cannot find value `CURRENT_MODULE` in this scope
   --> src/driver/jit.rs:159:13
    |
159 |             CURRENT_MODULE.with(|current_module| {


error[E0425]: cannot find value `CURRENT_MODULE` in this scope
   --> src/driver/jit.rs:183:13
    |
183 |             CURRENT_MODULE.with(|current_module| {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
