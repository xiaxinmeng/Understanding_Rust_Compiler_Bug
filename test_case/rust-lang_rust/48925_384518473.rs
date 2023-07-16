plain
[00:05:11]    Compiling backtrace v0.3.6
[00:05:12]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:12]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:15]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:05:27] error[E0609]: no field `fn_must_use` on type `&feature_gate::Features`
[00:05:27]     --> libsyntax/feature_gate.rs:1342:39
[00:05:27]      |
[00:05:27] 1342 |               gate_feature!(cx.context, $feature, span, $explain, $level)
[00:05:27] ...
[00:05:27] ...
[00:05:27] 1549 | /                     gate_feature_post!(&self, fn_must_use, attr.span,
[00:05:27] 1550 | |                                        "`#[must_use]` on functions is experimental",
[00:05:27] 1551 | |                                        GateStrength::Soft);
[00:05:27]      | |___________________________________________________________- in this macro invocation
[00:05:27] 
[00:05:27] error[E0609]: no field `fn_must_use` on type `&feature_gate::Features`
[00:05:27]     --> libsyntax/feature_gate.rs:1342:39
[00:05:27] 1342 
