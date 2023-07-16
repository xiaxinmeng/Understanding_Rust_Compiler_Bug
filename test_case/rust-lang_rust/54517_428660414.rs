plain
[00:49:21] .................................................................................................... 2200/4581
[00:49:25] ......i............................................................................................. 2300/4581
[00:49:29] .................................................................................................... 2400/4581
[00:49:32] .................................................................................................... 2500/4581
[00:49:36] ...................iiiiiiiii........................................................................ 2600/4581
[00:49:42] .................................................................................................... 2800/4581
[00:49:46] .................................................................................................... 2900/4581
[00:49:49] .......................................i............................................................ 3000/4581
[00:49:52] ...................................................................................................i 3100/4581
---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:59] 
[01:02:59] running 42 tests
[01:03:33] .................FF....F...........F.F....
[01:03:33] 
[01:03:33] ---- [ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs stdout ----
[01:03:33] diff of stderr:
[01:03:33] 
[01:03:33] 
[01:03:33] + error[E0428]: the name `Test` is defined multiple times
[01:03:33] +   --> $DIR/ambiguous-builtin-attrs-test.rs:13:1
[01:03:33] +    |
[01:03:33] + LL | #[test] // OK, shadowed
[01:03:33] +    | ------- previous definition of the type `Test` here
[01:03:33] + ...
[01:03:33] + LL | #[bench] // OK, shadowed
[01:03:33] +    | ^^^^^^^^ `Test` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Test` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0425]: cannot find value `Bench` in this scope
[01:03:33] +   --> $DIR/ambiguous-builtin-attrs-test.rs:18:5
[01:03:33] +    |
[01:03:33] + LL |     Bench;
[01:03:33] + 
[01:03:33] 1 error[E0425]: cannot find value `NonExistent` in this scope
[01:03:33] 2   --> $DIR/ambiguous-builtin-attrs-test.rs:19:5
[01:03:33] 3    |
[01:03:33] 3    |
[01:03:33] 
[01:03:33] 4 LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope
[01:03:33] 6 
[01:03:33] - error: aborting due to previous error
[01:03:33] + error: aborting due to 3 previous errors
[01:03:33] 8 
[01:03:33] 8 
[01:03:33] - For more information about this error, try `rustc --explain E0425`.
[01:03:33] + Some errors occurred: E0425, E0428.
[01:03:33] + For more information about an error, try `rustc --explain E0425`.
[01:03:33] 10 
[01:03:33] 
[01:03:33] 
[01:03:33] The actual stderr differed from the expected stderr.
[01:03:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/ambiguous-builtin-attrs-test.stderr
[01:03:33] To update references, rerun the tests and pass the `--bless` flag
[01:03:33] To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs-test.rs`
[01:03:33] error: 1 errors occurred comparing output.
[01:03:33] status: exit code: 1
[01:03:33] status: exit code: 1
[01:03:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "unused"
[01:03:33] ------------------------------------------
[01:03:33] 
[01:03:33] ------------------------------------------
[01:03:33] stderr:
[01:03:33] stderr:
[01:03:33] ------------------------------------------
[01:03:33] {"message":"the name `Test` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n