plain
travis_time:end:112ada08:start=1541380977566714492,finish=1541380979585902360,duration=2019187868
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:11] .................................................................................................... 4100/4988
[00:53:14] .....................................................................................i.............. 4200/4988
[00:53:20] .................................................................................................... 4300/4988
[00:53:23] .................................................................................................... 4400/4988
[00:53:26] ................................................................................................F..F 4500/4988
[00:53:34] .................................................................................................... 4700/4988
[00:53:37] .................................................................................................... 4800/4988
[00:53:37] .................................................................................................... 4800/4988
        ^^^^ slice indices are of type `usize` or ranges of `usize`
[00:53:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:53:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:53:42] -    = help: the trait `std::ops::Index<{integer}>` is not implemented for `str`
[00:53:42] +    = help: the trait `std::slice::SliceIndex<str>` is not implemented for `{integer}`
[00:53:42] +    = note: required because of the requirements on the impl of `std::ops::Index<{integer}>` for `str`
[00:53:42] 9 error: aborting due to previous error
[00:53:42] 10 
[00:53:42] 
[00:53:42] 
[00:53:42] 
[00:53:42] The actual stderr differed from the expected stderr.
[00:53:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/str-idx.stderr
[00:53:42] To update references, rerun the tests and pass the `--bless` flag
[00:53:42] To only update this specific test, also pass `--test-args str/str-idx.rs`
[00:53:42] error: 1 errors occurred comparing output.
[00:53:42] status: exit code: 1
[00:53:42] status: exit code: 1
[00:53:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-idx.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/auxiliary" "-A" "unused"
[00:53:42] ------------------------------------------
[00:53:42] 
[00:53:42] 
[00:53:42]en":[],"rendered":null}],"rendered":"error[E0277]: the trait bound `{integer}: std::slice::SliceIndex<str>` is not satisfied\n  --> /checkout/src/test/ui/str/str-idx.rs:13:17\n   |\nLL |     let c: u8 = s[4]; //~ ERROR the type `str` cannot be indexed by `{integer}`\n   |                 ^^^^ slice indices are of type `usize` or ranges of `usize`\n   |\n   = help: the trait `std::slice::SliceIndex<str>` is not implemented for `{integer}`\n   = note: required because of the requirements on the impl of `std::ops::Index<{integer}>` for `str`\n\n"}
[00:53:42] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:53:42] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[00:53:42] ------------------------------------------
[00:53:42] 
[00:53:42] thread '[ui] ui/str/str-idx.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:53:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:42] 
[00:53:42] ---- [ui] ui/str/str-mut-idx.rs stdout ----
[00:53:42] diff of stderr:
[00:53:42] 
[00:53:42] 22    = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:53:42] 23    = note: the left-hand-side of an assignment must have a statically known size
[00:53:42] 24 
[00:53:42] - error[E0277]: the type `str` cannot be mutably inde\ntrait Foo {\n    fn bar(&self);\n}\n\n// we now declare a function which takes an object implementing the Foo trait\nfn some_func<T: Foo>(foo: T) {\n    foo.bar();\n}\n\nfn main() {\n    // we now call the method with the i32 type, which doesn't implement\n    // the Foo trait\n    some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied\n}\n