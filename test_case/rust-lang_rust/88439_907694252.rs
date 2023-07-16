plain
    Checking cranelift-native v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-frontend v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git#5deda279)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0027]: pattern does not mention field `cleanup`
    |
376 | /             TerminatorKind::InlineAsm {
377 | |                 template,
378 | |                 operands,
378 | |                 operands,
379 | |                 options,
380 | |                 destination,
381 | |                 line_spans: _,
382 | |             } => {
    | |_____________^ missing field `cleanup`
help: include the missing field in the pattern
    |
    |
381 |                 line_spans: _, cleanup } => {
    |                              ^^^^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
    |
381 |                 line_spans: _, .. } => {

For more information about this error, try `rustc --explain E0027`.
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:29
