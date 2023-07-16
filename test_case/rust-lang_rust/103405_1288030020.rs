plain
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
error: unexpected `if` in the condition expression
   --> compiler/rustc_macros/src/diagnostics/utils.rs:622:30
    |
622 |                 *code_init = if let Some(init) = code.value() {
    |
help: remove the `if`
    |
    |
622 -                 *code_init = if let Some(init) = code.value() {
622 +                 *code_init = let Some(init) = code.value() {

error: expected expression, found `let` statement
   --> compiler/rustc_macros/src/diagnostics/utils.rs:622:33
    |
    |
622 |                 *code_init = if let Some(init) = code.value() {


error: expected one of `.`, `;`, `?`, `else`, `}`, or an operator, found `{`
    |
    |
622 |                 *code_init = if let Some(init) = code.value() {
    |                                                               ^ expected one of `.`, `;`, `?`, `else`, `}`, or an operator
error[E0658]: `let` expressions in this position are unstable
   --> compiler/rustc_macros/src/diagnostics/utils.rs:622:33
    |
    |
622 |                 *code_init = if let Some(init) = code.value() {
    |
    = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
    = help: add `#![feature(let_chains)]` to the crate attributes to enable

