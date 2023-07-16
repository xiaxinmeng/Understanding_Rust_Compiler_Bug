
[00:34:29]    Compiling clippy_lints v0.0.166 (file:///checkout/src/tools/clippy/clippy_lints)
[00:34:29] error: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:34:29]  --> <declare_lint macros>:1:3
[00:34:29]   |
[00:34:29] 1 | ( $ vis : vis $ NAME : ident , $ Level : ident , $ desc : expr ) => (
[00:34:29]   |   ^^^^^^^^^^^
[00:34:29]   |
[00:34:29]   = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:34:29] 
[00:34:29] error: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:34:29]  --> <declare_lint macros>:1:3
[00:34:29]   |
[00:34:29] 1 | ( $ vis : vis $ NAME : ident , $ Level : ident , $ desc : expr ) => (
[00:34:29]   |   ^^^^^^^^^^^
[00:34:29]   |
[00:34:29]   = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:34:29] 
[00:34:30] error: aborting due to previous error
[00:34:30] 
[00:34:30] error: Could not compile `clippy_lints`.
[00:34:30] warning: build failed, waiting for other jobs to finish...
[00:34:30] error: aborting due to previous error
[00:34:30] 
[00:34:30] error: Could not compile `clippy_lints`.
[00:34:30] warning: build failed, waiting for other jobs to finish...
[00:34:41] error: build failed
