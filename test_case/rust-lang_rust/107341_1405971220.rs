plain
[RUSTC-TIMING] fluent_bundle test:false 1.416
error: using `iter` can result in unstable query results
   --> compiler/rustc_macros/src/diagnostics/fluent.rs:279:43
    |
279 |         for (&mref, &name) in messagerefs.iter() {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
    = note: `-D rustc::potential-query-instability` implied by `-D warnings`
[RUSTC-TIMING] rustc_macros test:false 0.953
error: could not compile `rustc_macros` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_macros test:false 2.528
