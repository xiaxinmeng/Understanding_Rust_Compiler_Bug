plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:16] 
[00:47:16] running 1376 tests
[00:47:21] ...........................................................F......................i.................
[00:47:31] ....................................................................................................
[00:47:31] ....................................................................................................
[00:47:35] .............F......................................................................................
[00:47:43] ....................................................................................................
[00:47:49] ....................................................................................................
[00:47:49] ....................................................................................................
[00:47:55] ..................F.................................................................................
[00:48:08] ..................................i.................................................................
[00:48:13] ..........i.........................................................................................
[00:48:20] ...........................ii.......................................................................
[00:48:26] ....................................................................................................
[00:48:26] ....................................................................................................
[00:48:32] .......i........................F...F.......................................
[00:48:32] 
[00:48:32] ---- [ui] ui/chalkify/lower_env1.rs stdout ----
[00:48:32]  diff of stderr:
[00:48:32] 
[00:48:32] 
[00:48:32] 4 LL | #[rustc_dump_program_clauses] //~ ERROR program clause dump
[00:48:32] 6    |
[00:48:32] 6    |
[00:48:32] -    = note: FromEnv(Self: Bar) :- FromEnv(Self: Bar).
[00:48:32] 8    = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:48:32] +    = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:48:32] 9    = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).
[00:48:32] 10 
[00:48:32] 11 error: program clause dump
[00:48:32] 
[00:48:32] 14 LL | #[rustc_dump_env_program_clauses] //~ ERROR program clause dump
[00:48:32] 16    |
[00:48:32] 16    |
[00:48:32] -    = note: FromEnv(Self: Bar) :- FromEnv(Self: Bar).
[00:48:32] +    = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:48:32] 18    = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).
[00:48:32] 19    = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).
[00:48:32] 20    = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).
[00:48:32] 
[00:48:32] The actual stderr differed from the expected stderr.
[00:48:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1.stderr
[00:48:32] To update references, run this command from build directory:
[00:48:32] To update references, run this command from build directory:
[00:48:32] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'chalkify/lower_env1.rs'
[00:48:32] error: 1 errors occurred comparing output.
[00:48:32] status: exit code: 101
[00:48:32] status: exit code: 101
[00:48:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/lower_env1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/lower_env1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:32] ------------------------------------------
[00:48:32] ------------------------------------------
[00:48:32] program clause dump","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"FromEnv(Self: Foo) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"FromEnv(Self: Foo) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Bar) :- FromEnv(Self: Bar).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: Foo) :- FromEnv(Self: Foo).","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: program clause dump\n  --> /checkout/src/test/ui/chalkify/lower_env1.rs:19:1\n   |\nLL | #[rustc_dump_env_program_clauses] //~ ERROR program clause dump\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).\n   = note: FromEnv(Self: Foo) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Bar) :- FromEnv(Self: Bar).\n   = note: Implemented(Self: Foo) :- FromEnv(Self: Foo).\n   = note: Implemented(Self: std::marker::Sized) :- FromEnv(Self: std::marker::Sized).\n\n"}
[00:48:32] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:32] ------------------------------------------
[00:48:32] 
[00:48:32] thread '[ui] ui/chalkify/lower_env1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[00:48:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:32] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:32] 
[00:48:32] ---- [ui] ui/error-codes/E0275.rs stdout ----
[00:48:32]  
[00:48:32] error: ui test compiled successfully!
[00:48:32] status: exit code: 0
[00:48:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0275.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:32] ------------------------------------------
[00:48:32] 
[00:48:32] ------------------------------------------
[00:48:32] stderr:
---
[00:48:32] thread '[ui] ui/error-codes/E0275.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2963:9
[00:48:32] 
[00:48:32] ---- [ui] ui/issue-24424.rs stdout ----
[00:48:32]  
[00:48:32] error: ui test compiled successfully!
[00:48:32] status: exit code: 0
[00:48:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-24424.rs" "-L" "/checkout/obj:48:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check-defaults.stderr
[00:48:32] To update references, run this command from build directory:
[00:48:32] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'type-check-defaults.rs'
[00:48:32] error: 1 errors occurred comparing output.
[00:48:32] status: exit code: 101
[00:48:32] status: exit code: 101
[00:48:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-check-defaults.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check-defaults.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check-defaults.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:32] ------------------------------------------
[00:48:32] 
[00:48:32] ------------------------------------------
[00:48:32] stderr:
[00:48:32] stderr:
[00:48:32] ------------------------------------------
[00:48:32] {"message":"the trait bound `i32: std::iter::FromIterator<i32>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n