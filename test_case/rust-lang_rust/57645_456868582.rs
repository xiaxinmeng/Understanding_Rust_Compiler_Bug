plain
travis_time:end:2455da4a:start=1548256784400597872,finish=1548256873715853118,duration=89315255246
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:00] .................................................................................................... 2600/5322
[01:04:05] .................................................................................................... 2700/5322
[01:04:09] .................................................................................................... 2800/5322
[01:04:13] .................................................................................................... 2900/5322
[01:04:16] ...............................................................................F.................... 3000/5322
[01:04:23] ..............................................................................................i..... 3200/5322
[01:04:26] .................................................................................................... 3300/5322
[01:04:30] ...........................................................ii...i..ii............................... 3400/5322
[01:04:34] .................................................................................................... 3500/5322
---
[01:05:42] .............................................................i...................................... 5300/5322
[01:05:43] ......................
[01:05:43] failures:
[01:05:43] 
[01:05:43] ---- [ui] ui/layout/homogeneous-aggr-zero-sized-repr-rust.rs stdout ----
[01:05:43] 
[01:05:43] 
[01:05:43] - error: homogeneous_aggregate: Some(Reg { kind: Float, size: Size { raw: 4 } })
[01:05:43] -   --> $DIR/homogeneous-aggr-phantom-data.rs:31:1
[01:05:43] + error[E0428]: the name `Test3` is defined multiple times
[01:05:43] +   --> $DIR/homogeneous-aggr-zero-sized-repr-rust.rs:66:1
[01:05:43] 3    |
[01:05:43] - LL | pub type Test1 = Foo;
[01:05:43] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:43] + LL | pub type Test3 = WithEmptyRustStruct;
[01:05:43] +    | ------------------------------------- previous definition of the type `Test3` here
[01:05:43] + ...
[01:05:43] + LL | pub type Test3 = WithTransitivelyEmptyRustStruct;
[01:05:43] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Test3` redefined here
[01:05:43] +    |
[01:05:43] +    = note: `Test3` must be defined only once in the type namespace of this module
[01:05:43] 6 
[01:05:43] - error: homogeneous_aggregate: Some(Reg { kind: Float, size: Size { raw: 4 } })
[01:05:43] -   --> $DIR/homogeneous-aggr-phantom-data.rs:35:1
[01:05:43] + error[E0428]: the name `Test3` is defined multiple times
[01:05:43] +   --> $DIR/homogeneous-aggr-zero-sized-repr-rust.rs:70:1
[01:05:43] 9    |
[01:05:43] - LL | pub type Test2 = Bar;
[01:05:43] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:43] + LL | pub type Test3 = WithEmptyRustStruct;
[01:05:43] +    | ------------------------------------- previous definition of the type `Test3` here
[01:05:43] + ...
[01:05:43] + LL | pub type Test3 = WithEmptyRustEnum;
[01:05:43] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Test3` redefined here
[01:05:43] +    |
[01:05:43] +    = note: `Test3` must be defined only once in the type namespace of this module
[01:05:43] 12 
[01:05:43] - error: homogeneous_aggregate: Some(Reg { kind: Float, size: Size { raw: 4 } })
[01:05:43] -   --> $DIR/homogeneous-aggr-phantom-data.rs:39:1
[01:05:43] + error[E0412]: cannot find type `EmptyRust` in this scope
[01:05:43] +   --> $DIR/homogeneous-aggr-zero-sized-repr-rust.rs:27:16
[01:05:43] 15    |
[01:05:43] - LL | pub type Test3 = Baz;
[01:05:43] -    | ^^^^^^^^^^^^^^^^^^^^^
[01:05:43] + LL |     pub _unit: EmptyRust,
[01:05:43] 18 
[01:05:43] - error: aborting due to 3 previous errors
[01:05:43] - error: aborting due to 3 previous errors
[01:05:43] + error[E0412]: cannot find type `EmptyRust` in this scope
[01:05:43] +   --> $DIR/homogeneous-aggr-zero-sized-repr-rust.rs:39:16
[01:05:43] +    |
[01:05:43] + LL |     pub _unit: EmptyRust,
[01:05:43] 20 
[01:05:43] + error: aborting due to 4 previous errors
[01:05:43] + 
[01:05:43] + Some errors occurred: E0412, E0428.
[01:05:43] + Some errors occurred: E0412, E0428.
[01:05:43] + For more information about an error, try `rustc --explain E0412`.
[01:05:43] 21 
[01:05:43] 
[01:05:43] 
[01:05:43] The actual stderr differed from the expected stderr.
[01:05:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/homogeneous-aggr-zero-sized-repr-rust/homogeneous-aggr-zero-sized-repr-rust.stderr
[01:05:43] To update references, rerun the tests and pass the `--bless` flag
[01:05:43] To only update this specific test, also pass `--test-args layout/homogeneous-aggr-zero-sized-repr-rust.rs`
[01:05:43] error: 1 errors occurred comparing output.
[01:05:43] status: exit code: 1
[01:05:43] status: exit code: 1
[01:05:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/layout/homogeneous-aggr-zero-sized-repr-rust.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/homogeneous-aggr-zero-sized-repr-rust/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/layout/homogeneous-aggr-zero-sized-repr-rust/auxiliary" "-A" "unused"
[01:05:43] ------------------------------------------
[01:05:43] 
[01:05:43] ------------------------------------------
[01:05:43] stderr:
[01:05:43] stderr:
[01:05:43] ------------------------------------------
[01:05:43] {"message":"the name `Test3` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n