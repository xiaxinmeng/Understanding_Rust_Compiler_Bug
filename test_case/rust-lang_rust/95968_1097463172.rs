plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: unused import: `fluent_bundle`
  --> compiler/rustc_session/src/session.rs:23:29
   |
23 |     fallback_fluent_bundle, fluent_bundle, DiagnosticBuilder, DiagnosticId, DiagnosticMessage,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_session` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_session` due to previous error
Build completed unsuccessfully in 0:01:53
