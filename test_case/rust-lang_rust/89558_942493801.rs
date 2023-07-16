plain
   Compiling rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
   Compiling rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error: using `into_iter` can result in unstable query results
    --> compiler/rustc_errors/src/emitter.rs:1495:43
     |
1495 |                     for (depth, style) in &multilines {
     |
     |
     = note: `-D rustc::potential-query-instability` implied by `-D warnings`
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    --> compiler/rustc_errors/src/emitter.rs:1510:51
     |
1510 | ...                   for (depth, style) in &multilines {
     |
     |
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    --> compiler/rustc_errors/src/emitter.rs:1537:51
     |
1537 | ...                   for (depth, style) in &multilines {
     |
     |
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
     |
1032 |                 .iter()
     |                  ^^^^
     |
     |
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: could not compile `rustc_errors` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:06:35
