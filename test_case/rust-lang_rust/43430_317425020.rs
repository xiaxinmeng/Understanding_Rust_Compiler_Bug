
[00:01:17]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:01:18] error[E0560]: struct `dist::DebuggerScripts` has no field named `target`
[00:01:18]    --> /checkout/src/bootstrap/check.rs:538:17
[00:01:18]     |
[00:01:18] 538 |                 target: target
[00:01:18]     |                 ^^^^^^^ field does not exist - did you mean `host`?
[00:01:18] 
[00:01:19] error[E0560]: struct `dist::DebuggerScripts` has no field named `target`
[00:01:19]    --> /checkout/src/bootstrap/dist.rs:435:17
[00:01:19]     |
[00:01:19] 435 |                 target: host,
[00:01:19]     |                 ^^^^^^^ field does not exist - did you mean `host`?
[00:01:19] 
[00:01:19] error[E0560]: struct `dist::Extended` has no field named `compiler`
[00:01:19]     --> /checkout/src/bootstrap/dist.rs:1103:13
[00:01:19]      |
[00:01:19] 1103 |             compiler: run.builder.top_stage,
[00:01:19]      |             ^^^^^^^^^ field does not exist - did you mean `stage`?
