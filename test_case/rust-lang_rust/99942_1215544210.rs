`
     Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: passing `TypeError<'_>` by reference
  --> compiler/rustc_middle/src/ty/error.rs:78:29
   |
78 |     pub fn involves_regions(&self) -> bool {
   |                             ^^^^^ help: try passing by value: `TypeError<'_>`
   |
   = note: `-D rustc::pass-by-value` implied by `-D warnings`

error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
Build completed unsuccessfully in 0:02:05
