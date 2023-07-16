plain
[00:43:06] ....................................................................................................
[00:43:12] ....................................................................................................
[00:43:17] ....................................................................................................
[00:43:22] i..............................................................................i....................
[00:43:27] .........................................................................................F.F........
[00:43:39] ....................................................................................................
[00:43:39] ....................................................................................................
[00:43:44] ............i.................iiiiiiiii...................................................
[00:43:44] 
[00:43:44] ---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs stdout ----
[00:43:44] diff of stderr:
[00:43:44] 
[00:43:44] 
[00:43:44] 1 error: functions using `#[should_panic]` must return `()`
[00:43:44] -   --> $DIR/termination-trait-in-test-should-panic.rs:22:1
[00:43:44] +   --> $DIR/termination-trait-in-test-should-panic.rs:21:1
[00:43:44] 3    |
[00:43:44] 4 LL | / fn not_a_num() -> Result<(), ParseIntError> {
[00:43:44] 5 LL | |     //~^ ERROR functions using `#[should_panic]` must retrait/termination-trait-test-wrong-type.rs stdout ----
[00:43:44] 
[00:43:44] 
[00:43:44] 1 error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseIntError>`
[00:43:44] -   --> $DIR/termination-trait-test-wrong-type.rs:18:1
[00:43:44] +   --> $DIR/termination-trait-test-wrong-type.rs:16:1
[00:43:44] 3    |
[00:43:44] 4 LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseIntError> { //~ ERROR
[00:43:44] 5 LL | |     "0".parse()
[00:43:44] 
[00:43:44] The actual stderr differed from the expected stderr.
[00:43:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
[00:43:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
[00:43:44] To update references, rerun the tests and pass the `--bless` flag
[00:43:44] To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
[00:43:44] error: 1 errors occurred comparing output.
[00:43:44] status: exit code: 101
[00:43:44] status: exit code: 101
[00:43:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
[00:43:44] ------------------------------------------
[00:43:44] 
[00:43:44] ------------------------------------------
[00:43:44] stderr:
[00:43:44] stderr:
[00:43:44] ------------------------------------------
[00:43:44] {"message":"`main` has invalid return type `std::result::Result<f32, std::num::ParseIntError>`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n