plain
travis_time:end:0ef62f00:start=1553481141053032786,finish=1553481143474909505,duration=2421876719
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:30] 
[01:10:30] running 5511 tests
[01:10:35] .......................................................................................F............ 100/5511
[01:10:41] .................................................................................................... 300/5511
[01:10:44] .................................................................................................... 400/5511
[01:10:47] .................................................................................................... 500/5511
[01:10:50] ..................................................................i................................. 600/5511
---
[01:11:11] ........................................................iiiii....................................... 1100/5511
[01:11:15] .................................................................................................... 1200/5511
[01:11:17] .................................................................................................... 1300/5511
[01:11:20] .................................................................................................... 1400/5511
[01:11:23] ....................F...........................F................................................... 1500/5511
[01:11:25] ...............................F.................................................................... 1600/5511
[01:11:32] .................................................................................................... 1800/5511
[01:11:36] .................................................................................................... 1900/5511
[01:11:39] .................................................................................................... 2000/5511
[01:11:43] .................................................................................................... 2100/5511
---
[01:13:27] .................................................................................................... 4800/5511
[01:13:30] .................................................................................................... 4900/5511
[01:13:33] .................................................................................................... 5000/5511
[01:13:38] .................................................................................................... 5100/5511
[01:13:41] ....................................................................................F............... 5200/5511
[01:13:47] .................................................................................................... 5400/5511
[01:13:50] .................................................i.................................................. 5500/5511
[01:13:50] ...........
[01:13:50] failures:
[01:13:50] failures:
[01:13:50] 
[01:13:50] ---- [ui] ui/associated-type-bounds/nested-lifetime-bounds.rs stdout ----
[01:13:50] 
[01:13:50] error: /checkout/src/test/ui/associated-type-bounds/nested-lifetime-bounds.rs:11: expected error not found: nested quantification of lifetimes [E0316]
[01:13:50] error: 0 unexpected errors found, 1 expected errors not found
[01:13:50] status: exit code: 1
[01:13:50] status: exit code: 1
[01:13:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/nested-lifetime-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/nested-lifetime-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/nested-lifetime-bounds/auxiliary" "-A" "unused"
[01:13:50]     Error {
[01:13:50]         line_num: 11,
[01:13:50]         kind: Some(
[01:13:50]             Error
[01:13:50]             Error
[01:13:50]         ),
[01:13:50]         msg: "nested quantification of lifetimes [E0316]"
[01:13:50] ]
[01:13:50] 
[01:13:50] thread '[ui] ui/associated-type-bounds/nested-lifetime-bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1381:13
[01:13:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:50] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:50] 
[01:13:50] ---- [ui] ui/existential_types/declared_but_not_defined_in_scope.rs stdout ----
[01:13:50] 
[01:13:50] error: ui test compiled successfully!
[01:13:50] status: exit code: 0
[01:13:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/declared_but_not_defined_in_scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/declared_but_not_defined_in_scope/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/declared_but_not_defined_in_scope/auxiliary" "-A" "unused"
[01:13:50] ------------------------------------------
[01:13:50] 
[01:13:50] ------------------------------------------
[01:13:50] stderr:
---
[01:13:50] -   --> $DIR/no_revealing_outside_defining_module.rs:19:5
[01:13:50] + error: concrete type differs from previous defining existential type use
[01:13:50] +   --> $DIR/no_revealing_outside_defining_module.rs:22:1
[01:13:50] 12    |
[01:13:50] - LL | fn bomp() -> boo::Boo {
[01:13:50] -    |              -------- expected `Boo` because of return type
[01:13:50] - LL |     ""
[01:13:50] -    |     ^^ expected opaque type, found reference
[01:13:50] + LL | / fn bomp_loop() -> boo::Boo {
[01:13:50] + LL | |     loop {}
[01:13:50] + LL | | }
[01:13:50] +    | |_^ expected `&'static str`, got `()`
[01:13:50] -    = note: expected type `Boo`
[01:13:50] -               found type `&'static str`
[01:13:50] + note: previous use here
[01:13:50] +   --> $DIR/no_revealing_outside_defining_module.rs:7:5
[01:13:50] +   --> $DIR/no_revealing_outside_defining_module.rs:7:5
[01:13:50] +    |
[01:13:50] + LL | /     fn bomp() -> Boo {
[01:13:50] + LL | |         ""
[01:13:50] + LL | |     }
[01:13:50] 20 
[01:13:50] 21 error: aborting due to 2 previous errors
[01:13:50] 22 
[01:13:50] 
[01:13:50] 
[01:13:50] 
[01:13:50] The actual stderr differed from the expected stderr.
[01:13:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_revealing_outside_defining_module/no_revealing_outside_defining_module.stderr
[01:13:50] To update references, rerun the tests and pass the `--bless` flag
[01:13:50] To only update this specific test, also pass `--test-args existential_types/no_revealing_outside_defining_module.rs`
[01:13:50] error: 1 errors occurred comparing output.
[01:13:50] status: exit code: 1
[01:13:50] status: exit code: 1
[01:13:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_revealing_outside_defining_module/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_revealing_outside_defining_module/auxiliary" "-A" "unused"
[01:13:50] ------------------------------------------
[01:13:50] 
[01:13:50] ------------------------------------------
[01:13:50] stderr:
[01:13:50] stderr:
[01:13:50] ------------------------------------------
[01:13:50] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n