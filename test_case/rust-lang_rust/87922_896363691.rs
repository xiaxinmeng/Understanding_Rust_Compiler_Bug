plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: `if` and `else` have incompatible types
   --> compiler/rustc_middle/src/ty/layout.rs:134:13
    |
128 |           let at_least = if repr.c() {
    |  ________________________-
129 | |             // This is usually I32, however it can be different on some platforms,
130 | |             // notably hexagon and arm-none/thumb-none
131 | |             tcx.data_layout().c_enum_min_size;
    | |             |                                |
    | |             |                                help: consider removing this semicolon
    | |             expected because of this
...   |
...   |
134 | |             I8
    | |             ^^ expected `()`, found enum `rustc_target::abi::Integer`
135 | |         };
    | |_________- `if` and `else` have incompatible types
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle` due to previous error
Build completed unsuccessfully in 0:01:44
