plain
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking tracing-tree v0.1.9
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking chalk-solve v0.55.0
error: no rules expected the token `)`
   --> compiler/rustc_errors/src/diagnostic_builder.rs:245:42
41  | macro_rules! forward {
    | -------------------- when calling this macro
...
...
245 |     forward!(pub fn set_is_lint(&mut self) -> &mut Self);
    |                                          ^ no rules expected this token in macro call
error: could not compile `rustc_errors` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:04
