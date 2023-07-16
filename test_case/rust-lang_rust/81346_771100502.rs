plain
    Checking cranelift-frontend v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-jit v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-object v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `CCmseNonSecureCall` not covered
   --> src/abi/mod.rs:26:27
    |
26  |     let call_conv = match fn_abi.conv {
    |                           ^^^^^^^^^^^ pattern `CCmseNonSecureCall` not covered
   ::: /checkout/compiler/rustc_target/src/abi/call/mod.rs:554:5
    |
    |
554 |     CCmseNonSecureCall,
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Conv`

