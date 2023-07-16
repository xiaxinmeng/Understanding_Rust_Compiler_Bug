plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:51] 
[00:44:51] running 6782 tests
[00:44:54] ....................................................................................................
[00:44:56] ....................................................................F...............................
[00:45:02] ....................................................................................................
[00:45:05] ....................................................................................................
[00:45:09] .................i..................................................................................
[00:45:13] ....................................................................................................
---
[00:46:24] ............................................i.......................................................
[00:46:27] ....................................................................................................
[00:46:30] ....................................................................................................
[00:46:32] ....................................................................................................
[00:46:35] .............................................................FFFFFFF..................i.............
[00:46:49] ....................................................................................................
[00:46:56] ....................................................................................................
[00:47:03] ....................................................................................................
[00:47:12] ....................................................................................................
---
[00:51:16] 
[00:51:16] ---- [ui] ui/bind-by-move/bind-by-move-no-guards.rs stdout ----
[00:51:16] diff of stderr:
[00:51:16] 
[00:51:16] 1 error[E0008]: cannot bind by-move into a pattern guard
[00:51:16] +   --> $DIR/bind-by-move-no-guards.rs:8:14
[00:51:16] 3    |
[00:51:16] 3    |
[00:51:16] 4 LL |         Some(z) if z.recv().unwrap() => { panic!() },
[00:51:16] 5    |              ^ moves value into pattern guard
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux:15,"is_primary":true,"text":[{"text":"        Some(z) if z.recv().unwrap() => { panic!() },","highlight_start":14,"highlight_end":15}],"label":"moves value into pattern guard","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0008]: cannot bind by-move into a pattern guard\n  --> /checkout/src/test/ui/bind-by-move/bind-by-move-no-guards.rs:8:14\n   |\nLL |         Some(z) if z.recv().unwrap() => { panic!() },\n   |              ^ moves value into pattern guard\n\n"}
[00:51:16] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:16] {"message":"For more information about this error, try `rustc --explain E0008`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0008`.\n"}
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] thread '[ui] ui/bind-by-move/bind-by-move-no-guards.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:51:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:16] 
[00:51:16] ---- [ui] ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs#gate_and_2015 stdout ----
[00:51:16] 
[00:51:16] 
[00:51:16] 1 error[E0008]: cannot bind by-move into a pattern guard
[00:51:16] -   --> $DIR/feature-gate.rs:43:16
[00:51:16] +   --> $DIR/feature-gate.rs:33:16
[00:51:16] 3    |
[00:51:16] 4 LL |         A { a: v } if *v == 42 => v,
[00:51:16] 5    |                ^ moves value into pattern guard
[00:51:16] 
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] The actual stderr differed from the expected stderr.
[00:51:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.gate_and_2015/feature-gate.gate_and_2015.stderr
[00:51:16] To update references, rerun the tests and pass the `--bless` flag
[00:51:16] To only update this specific test, also pass `--test-args rfc-0107-bind-by-move-pattern-guards/feature-gate.rs`
[00:51:16] 
[00:51:16] error in revision `gate_and_2015`: 1 errors occurred comparing output.
[00:51:16] status: exit code: 1
[00:51:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "gate_and_2015" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.gate_and_2015/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/feature-gate.gate_and_2015/auxiliary" "-A" "unused"
[00:51:16] ------------------------------------------
[00:51:16] 
[00:51:16] ------------------------------------------
[00:51:16] stderr:
[00:51:16] stderr:
[00:51:16] ------------------------------------------
[00:51:16] {"message":"cannot bind by-move into a pattern guard","code":{"code":"E0008","explanation":"\nNames bound in match arms retain their type in pattern guards. As such, if a\nname is bound by move in a pattern, it should also be moved to wherever it is\nreferenced in the pattern guard code. Doing so however would prevent the name\nfrom being available in the body of the match arm. Consider the following:\n\n