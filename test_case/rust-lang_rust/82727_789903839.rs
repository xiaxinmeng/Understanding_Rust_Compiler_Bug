plain
    Checking cranelift-frontend v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-jit v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking cranelift-object v0.69.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#986b5768)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0658]: box pattern syntax is experimental
    |
    |
467 |                 Rvalue::BinaryOp(bin_op, box (ref lhs, ref rhs)) => {
    |
    = note: see issue #29641 <https://github.com/rust-lang/rust/issues/29641> for more information
    = help: add `#![feature(box_patterns)]` to the crate attributes to enable


error[E0658]: box pattern syntax is experimental
    |
    |
474 |                 Rvalue::CheckedBinaryOp(bin_op, box (ref lhs, ref rhs)) => {
    |
    = note: see issue #29641 <https://github.com/rust-lang/rust/issues/29641> for more information
    = help: add `#![feature(box_patterns)]` to the crate attributes to enable

