plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: expected identifier, found keyword `let`
   --> compiler/rustc_builtin_macros/src/test.rs:368:9
    |
368 |         let sd = &cx.sess.parse_sess.span_diagnostic;


error: expected one of `=>`, `@`, `if`, or `|`, found `sd`
    |
    |
368 |         let sd = &cx.sess.parse_sess.span_diagnostic;
    |             ^^ expected one of `=>`, `@`, `if`, or `|`
error: could not compile `rustc_builtin_macros` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:22
