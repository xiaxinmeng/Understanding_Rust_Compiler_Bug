plain
   Compiling tracing-tree v0.1.9
   Compiling chalk-solve v0.55.0
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error: using `into_iter` can result in unstable query results
  --> compiler/rustc_session/src/cgu_reuse_tracker.rs:92:17
   |
92 |                 &data.expected_reuse
   |
   |
   = note: `-D rustc::potential-query-instability` implied by `-D warnings`
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `drain` can result in unstable query results
   |
   |
55 |         for (gate, mut gate_spans) in inner.drain() {
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   |
   |
82 |         let mut sorted: Vec<_> = type_sizes.iter().collect();
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
885 |     cfg.into_iter().map(|(a, b)| (Symbol::intern(&a), b.map(|b| Symbol::intern(&b)))).collect()
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
    |
896 |     user_cfg.extend(default_cfg.iter().cloned());
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: could not compile `rustc_session` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:05:41
