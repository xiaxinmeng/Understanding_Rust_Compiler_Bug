plain
travis_time:end:00a18200:start=1548539985716498917,finish=1548540060997837736,duration=75281338819
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:07:24] .................................................................................................... 3000/5351
[01:07:27] .................................................................................................... 3100/5351
[01:07:32] .................................................................................................... 3200/5351
[01:07:35] ............i....................................................................................... 3300/5351
[01:07:39] ..................................F...........................................ii...i..ii............ 3400/5351
[01:07:47] .................................................................................................... 3600/5351
[01:07:50] ..........................................................................ii........................ 3700/5351
[01:07:53] ............................................................................................i....... 3800/5351
[01:07:55] .................................................................................................... 3900/5351
---
[01:08:54] normalized stderr:
[01:08:54] error[E0308]: mismatched types
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:85:24
[01:08:54]    |
[01:08:54] LL |     let _seetype: () = z; //~ ERROR mismatched types
[01:08:54]    |                        ^ expected (), found u32
[01:08:54]    = note: expected type `()`
[01:08:54]               found type `u32`
[01:08:54] 
[01:08:54] error[E0308]: mismatched types
[01:08:54] error[E0308]: mismatched types
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:102:24
[01:08:54]    |
[01:08:54] LL |     let _seetype: () = z; //~ ERROR mismatched types
[01:08:54]    |                        ^ expected (), found u64
[01:08:54]    = note: expected type `()`
[01:08:54]               found type `u64`
[01:08:54] 
[01:08:54] error[E0034]: multiple applicable items in scope
[01:08:54] error[E0034]: multiple applicable items in scope
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:120:15
[01:08:54]    |
[01:08:54] LL |     let z = x.foo(); //~ ERROR multiple applicable items in scope
[01:08:54]    |               ^^^ multiple `foo` found
[01:08:54]    |
[01:08:54] note: candidate #1 is defined in an impl of the trait `internal::X` for the type `_`
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:43:9
[01:08:54]    |
[01:08:54] LL |         fn foo(self: Smaht<Self, u64>) -> u64 {
[01:08:54]    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:08:54] note: candidate #2 is defined in an impl of the trait `nuisance_foo::NuisanceFoo` for the type `_`
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:70:9
[01:08:54] LL |         fn foo(self) {}
[01:08:54]    |         ^^^^^^^^^^^^
[01:08:54]    |         ^^^^^^^^^^^^
[01:08:54] note: candidate #3 is defined in the trait `FinalFoo`
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:57:5
[01:08:54] LL |     fn foo(&self) -> u8;
[01:08:54]    |     ^^^^^^^^^^^^^^^^^^^^
[01:08:54]    |     ^^^^^^^^^^^^^^^^^^^^
[01:08:54]    = help: to disambiguate the method call, write `FinalFoo::foo(x)` instead
[01:08:54] error[E0308]: mismatched types
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:137:24
[01:08:54]    |
[01:08:54]    |
[01:08:54] LL |     let _seetype: () = z; //~ ERROR mismatched types
[01:08:54]    |                        ^ expected (), found u8
[01:08:54]    = note: expected type `()`
[01:08:54]               found type `u8`
[01:08:54] 
[01:08:54] error[E0308]: mismatched types
[01:08:54] error[E0308]: mismatched types
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:155:24
[01:08:54]    |
[01:08:54] LL |     let _seetype: () = z; //~ ERROR mismatched types
[01:08:54]    |                        ^ expected (), found u32
[01:08:54]    = note: expected type `()`
[01:08:54]               found type `u32`
[01:08:54] 
[01:08:54] error[E0308]: mismatched types
[01:08:54] error[E0308]: mismatched types
[01:08:54]   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:172:24
[01:08:54]    |
[01:08:54] LL |     let _seetype: () = z; //~ ERROR mismatched types
[01:08:54]    |                        ^ expected (), found u32
[01:08:54]    = note: expected type `()`
[01:08:54]               found type `u32`
[01:08:54] 
[01:08:54] error: aborting due to 6 previous errors
[01:08:54] error: aborting due to 6 previous errors
[01:08:54] 
[01:08:54] Some errors occurred: E0034, E0308.
[01:08:54] For more information about an error, try `rustc --explain E0034`.
[01:08:54] 
[01:08:54] 
[01:08:54] 
[01:08:54] The actual stderr differed from the expected stderr.
[01:08:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/method-deref-to-same-trait-object-with-separate-params.stderr
[01:08:54] To update references, rerun the tests and pass the `--bless` flag
[01:08:54] To only update this specific test, also pass `--test-args methods/method-deref-to-same-trait-object-with-separate-params.rs`
[01:08:54] error: 1 errors occurred comparing output.
[01:08:54] status: exit code: 1
[01:08:54] status: exit code: 1
[01:08:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/auxiliary" "-A" "unused"
[01:08:54] ------------------------------------------
[01:08:54] 
[01:08:54] ------------------------------------------
[01:08:54] stderr:
[01:08:54] stderr:
[01:08:54] ------------------------------------------
[01:08:54] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n