plain
[00:46:16] 
[00:46:16] running 1470 tests
[00:46:20] .........................................................................................i..........
[00:46:26] ...............................................i....................................................
[00:46:30] ...................................................................FF....................F..........
[00:46:36] ....................................................................................................
[00:46:39] ....................................................................................................
[00:46:44] ....................................................................................................
[00:46:48] ....................................................................................................
[00:46:48] ....................................................................................................
[00:46:52] ....................................................................................................
[00:46:57] ......................................................................................i.............
[00:47:02] ...............................................................i....................................
[00:47:06] ....................................................................................................
[00:47:12] ....................................................................................................
[00:47:18] ............................................................................................i.......
[00:47:21] ..........iiiiiiiii...................................................
[00:47:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:21] 
[00:47:21] ---- [ui] ui/error-codes/E0087.rs stdout ----
[00:47:21] diff of stderr:
[00:47:21] diff of stderr:
[00:47:21] 
[00:47:21] 2   --> $DIR/E0087.rs:15:11
[00:47:21] 3    |
[00:47:21] 4 LL |     foo::<f64>(); //~ ERROR expected at most 0 type parameters, found 1 type parameter [E0087]
[00:47:21] -    |           ^^^ expected 0 type parameters
[00:47:21] +    |     ------^^^- expected 0 type parameters
[00:47:21] 6 
[00:47:21] 7 error[E0087]: too many type parameters provided: expected at most 1 type parameter, found 2 type parameters
[00:47:21] 8   --> $DIR/E0087.rs:17:16
[00:47:21] 9    |
[00:47:21] 9    |
[00:47:21] 10 LL |     bar::<f64, u64>(); //~ ERROR expected at most 1 type parameter, found 2 type parameters [E0087]
[00:47:21] -    |                ^^^ expected 1 type parameter
[00:47:21] +    |     -----------^^^- expected 1 type parameter
[00:47:21] 13 error: aborting due to 2 previous errors
[00:47:21] 14 
[00:47:21] 
[00:47:21] 
[00:47:21] 
[00:47:21] The actual stderr differed from the expected stderr.
[00:47:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0087/E0087.stderr
[00:47:21] To update references, rerun the tests and pass the `--bless` flag
[00:47:21] To only update this specific test, also pass `--test-args error-codes/E0087.rs`
[00:47:21] error: 1 errors occurred comparing output.
[00:47:21] status: exit code: 101
[00:47:21] status: exit code: 101
[00:47:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0087.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0087/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0087/auxiliary" "-A" "unused"
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] ------------------------------------------
[00:47:21] stderr:
[00:47:21] stderr:
[00:47:21] ------------------------------------------
[00:47:21] {"message":"too many type parameters provided: expected at most 0 type parameters, found 1 type parameter","code":{"code":"E0087","explanation":"\nToo many type parameters were supplied for a function. For example:\n\n