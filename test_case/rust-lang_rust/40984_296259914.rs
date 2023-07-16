
[00:17:04] Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
... snip ...
[00:17:20]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:17:20] error: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:17:20]  --> <thread_local macros>:2:29
[00:17:20]   |
[00:17:20] 2 | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:17:20]   |                             ^^^^^^^^^^^
[00:17:20]   |
[00:17:20]   = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:17:20] 
[00:17:20] error: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:17:20]  --> <thread_local macros>:6:29
[00:17:20]   |
[00:17:20] 6 | $ ( # [ $ attr : meta ] ) * $ vis : vis static $ name : ident : $ t : ty = $
[00:17:20]   |                             ^^^^^^^^^^^
[00:17:20]   |
[00:17:20]   = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
