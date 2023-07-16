plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0004]: non-exhaustive patterns: `RustCold` not covered
   --> src/abi/mod.rs:24:27
    |
24  |     let call_conv = match fn_abi.conv {
    |                           ^^^^^^^^^^^ pattern `RustCold` not covered
note: `Conv` defined here
   --> /checkout/compiler/rustc_target/src/abi/call/mod.rs:585:5
    |
577 | / pub enum Conv {
577 | / pub enum Conv {
578 | |     // General language calling conventions, for which every target
579 | |     // should have its own backend (e.g. LLVM) support.
...   |
585 | |     RustCold,
    | |     ^^^^^^^^ not covered
...   |
...   |
607 | |     AvrNonBlockingInterrupt,
    | |_-
    = note: the matched value is of type `Conv`
    = note: the matched value is of type `Conv`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
39  ~         | Conv::AvrNonBlockingInterrupt => todo!("{:?}", fn_abi.conv),
40  ~         RustCold => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:58
