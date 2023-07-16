plain
[00:49:56] .................................................................................................... 2200/4940
[00:50:00] .................................i.................................................................. 2300/4940
[00:50:04] .................................................................................................... 2400/4940
[00:50:07] .................................................................................................... 2500/4940
[00:50:11] ................................................iiiiiiiii........................................... 2600/4940
[00:50:15] ..................................................................................................ii 2700/4940
[00:50:21] .................................................................................................... 2900/4940
[00:50:24] ........................................................................................i........... 3000/4940
[00:50:27] .................................................................................................... 3100/4940
[00:50:30] ...............................................i.i..ii.............................................. 3200/4940
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:30] 
[01:03:30] running 111 tests
[01:03:33] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:03:33] ..ii.ii....
[01:03:33] 
[01:03:33]  finished in 3.584
[01:03:33] travis_fold:end:test_codegen

---
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:18] 
[01:04:18] running 42 tests
[01:04:51] .................FF....F...........F.F....
[01:04:51] 
[01:04:51] ---- [ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs stdout ----
[01:04:51] diff of stderr:
[01:04:51] 
[01:04:51] 
[01:04:51] + error[E0428]: the name `Test` is defined multiple times
[01:04:51] +   --> $DIR/ambiguous-builtin-attrs-test.rs:13:1
[01:04:51] +    |
[01:04:51] + LL | #[test] // OK, shadowed
[01:04:51] +    | ------- previous definition of the type `Test` here
[01:04:51] + ...
[01:04:51] + LL | #[bench] // OK, shadowed
[01:04:51] +    | ^^^^^^^^ `Test` redefined here
[01:04:51] +    |
[01:04:51] +    = note: `Test` must be defined only once in the type namespace of this module
[01:04:51] + 
[01:04:51] + error[E0425]: cannot find value `Bench` in this scope
[01:04:51] +   --> $DIR/ambiguous-builtin-attrs-test.rs:18:5
[01:04:51] +    |
[01:04:51] + LL |     Bench;
[01:04:51] + 
[01:04:51] 1 error[E0425]: cannot find value `NonExistent` in this scope
[01:04:51] 2   --> $DIR/ambiguous-builtin-attrs-test.rs:19:5
[01:04:51] 3    |
[01:04:51] 3    |
[01:04:51] 
[01:04:51] 4 LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope
[01:04:51] 6 
[01:04:51] - error: aborting due to previous error
[01:04:51] + error: aborting due to 3 previous errors
[01:04:51] 8 
[01:04:51] 8 
[01:04:51] - For more information about this error, try `rustc --explain E0425`.
[01:04:51] + Some errors occurred: E0425, E0428.
[01:04:51] + For more information about an error, try `rustc --explain E0425`.
[01:04:51] 10 
[01:04:51] 
[01:04:51] 
[01:04:51] The actual stderr differed from the expected stderr.
[01:04:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/ambiguous-builtin-attrs-test.stderr
[01:04:51] To update references, rerun the tests and pass the `--bless` flag
[01:04:51] To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs-test.rs`
[01:04:51] error: 1 errors occurred comparing output.
[01:04:51] status: exit code: 1
[01:04:51] status: exit code: 1
[01:04:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "unused"
[01:04:51] ------------------------------------------
[01:04:51] 
[01:04:51] ------------------------------------------
[01:04:51] stderr:
[01:04:51] stderr:
[01:04:51] ------------------------------------------
[01:04:51] {"message":"the name `Test` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n