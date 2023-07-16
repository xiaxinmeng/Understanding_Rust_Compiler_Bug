plain
travis_time:end:1d606d48:start=1553461214785743756,finish=1553461217156164865,duration=2370421109
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
[01:15:12] 
[01:15:12] running 5510 tests
[01:15:17] .......................................................................................F............ 100/5510
[01:15:23] .................................................................................................... 300/5510
[01:15:26] .................................................................................................... 400/5510
[01:15:29] .................................................................................................... 500/5510
[01:15:33] ..................................................................i................................. 600/5510
---
[01:15:55] .......................................................iiiii........................................ 1100/5510
[01:15:59] .................................................................................................... 1200/5510
[01:16:01] .................................................................................................... 1300/5510
[01:16:04] .................................................................................................... 1400/5510
[01:16:07] ...................F............................F................................................... 1500/5510
[01:16:10] ..............................F..................................................................... 1600/5510
[01:16:17] .................................................................................................... 1800/5510
[01:16:21] .................................................................................................... 1900/5510
[01:16:24] .................................................................................................... 2000/5510
[01:16:28] ..................................................................................................i. 2100/5510
---
[01:16:56] .................................................................................................... 2800/5510
[01:17:01] .................................................................................................... 2900/5510
[01:17:05] .................................................................................................... 3000/5510
[01:17:09] .................................................................................................... 3100/5510
[01:17:12] ....................................................................................F............... 3200/5510
[01:17:20] ............................i....................................................................... 3400/5510
[01:17:24] .................................................................................................... 3500/5510
[01:17:27] ..ii...i..ii........................................................................................ 3600/5510
[01:17:31] .................................................................................................... 3700/5510
---
[01:18:17] .................................................................................................... 4800/5510
[01:18:21] .................................................................................................... 4900/5510
[01:18:24] .................................................................................................... 5000/5510
[01:18:29] .................................................................................................... 5100/5510
[01:18:32] ....................................................................................F............... 5200/5510
[01:18:38] .................................................................................................... 5400/5510
[01:18:41] ................................................i................................................... 5500/5510
[01:18:42] ..........
[01:18:42] failures:
[01:18:42] failures:
[01:18:42] 
[01:18:42] ---- [ui] ui/associated-type-bounds/nested-lifetime-bounds.rs stdout ----
[01:18:42] 
[01:18:42] error: /checkout/src/test/ui/associated-type-bounds/nested-lifetime-bounds.rs:11: expected error not found: nested quantification of lifetimes [E0316]
[01:18:42] error: 0 unexpected errors found, 1 expected errors not found
[01:18:42] status: exit code: 1
[01:18:42] status: exit code: 1
[01:18:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/nested-lifetime-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/nested-lifetime-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/nested-lifetime-bounds/auxiliary" "-A" "unused"
[01:18:42]     Error {
[01:18:42]         line_num: 11,
[01:18:42]         kind: Some(
[01:18:42]             Error
[01:18:42]             Error
[01:18:42]         ),
[01:18:42]         msg: "nested quantification of lifetimes [E0316]"
[01:18:42] ]
[01:18:42] 
[01:18:42] thread '[ui] ui/associated-type-bounds/nested-lifetime-bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1381:13
[01:18:42] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:42] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:18:42] 
[01:18:42] ---- [ui] ui/existential_types/declared_but_not_defined_in_scope.rs stdout ----
[01:18:42] 
[01:18:42] error: ui test compiled successfully!
[01:18:42] status: exit code: 0
[01:18:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/declared_but_not_defined_in_scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/declared_but_not_defined_in_scope/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/declared_but_not_defined_in_scope/auxiliary" "-A" "unused"
[01:18:42] ------------------------------------------
[01:18:42] 
[01:18:42] ------------------------------------------
[01:18:42] stderr:
---
[01:18:42] -   --> $DIR/no_revealing_outside_defining_module.rs:19:5
[01:18:42] + error: concrete type differs from previous defining existential type use
[01:18:42] +   --> $DIR/no_revealing_outside_defining_module.rs:22:1
[01:18:42] 12    |
[01:18:42] - LL | fn bomp() -> boo::Boo {
[01:18:42] -    |              -------- expected `Boo` because of return type
[01:18:42] - LL |     ""
[01:18:42] -    |     ^^ expected opaque type, found reference
[01:18:42] + LL | / fn bomp_loop() -> boo::Boo {
[01:18:42] + LL | |     loop {}
[01:18:42] + LL | | }
[01:18:42] +    | |_^ expected `&'static str`, got `()`
[01:18:42] -    = note: expected type `Boo`
[01:18:42] -               found type `&'static str`
[01:18:42] + note: previous use here
[01:18:42] +   --> $DIR/no_revealing_outside_defining_module.rs:7:5
[01:18:42] +   --> $DIR/no_revealing_outside_defining_module.rs:7:5
[01:18:42] +    |
[01:18:42] + LL | /     fn bomp() -> Boo {
[01:18:42] + LL | |         ""
[01:18:42] + LL | |     }
[01:18:42] 20 
[01:18:42] 21 error: aborting due to 2 previous errors
[01:18:42] 22 
[01:18:42] 
[01:18:42] 
[01:18:42] 
[01:18:42] The actual stderr differed from the expected stderr.
[01:18:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_revealing_outside_defining_module/no_revealing_outside_defining_module.stderr
[01:18:42] To update references, rerun the tests and pass the `--bless` flag
[01:18:42] To only update this specific test, also pass `--test-args existential_types/no_revealing_outside_defining_module.rs`
[01:18:42] error: 1 errors occurred comparing output.
[01:18:42] status: exit code: 1
[01:18:42] status: exit code: 1
[01:18:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/no_revealing_outside_defining_module.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_revealing_outside_defining_module/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_revealing_outside_defining_module/auxiliary" "-A" "unused"
[01:18:42] ------------------------------------------
[01:18:42] 
[01:18:42] ------------------------------------------
[01:18:42] stderr:
[01:18:42] stderr:
[01:18:42] ------------------------------------------
[01:18:42] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n