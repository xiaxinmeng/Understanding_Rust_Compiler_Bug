plain
travis_time:end:04a92f3a:start=1545296757612708936,finish=1545296758708392864,duration=1095683928
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:53:36] ..............................i..................................................................... 600/5189
[00:53:39] .................................................................................................... 700/5189
[00:53:44] .................................................................................................... 800/5189
[00:53:49] ......i...............i............................................................................. 900/5189
[00:53:51] ...........................F..iiiii................................................................. 1000/5189
[00:53:54] ........................................................................F........................... 1100/5189
[00:53:56] .........................................................F.......................................... 1200/5189
[00:54:01] .................................................................................................... 1400/5189
[00:54:03] .................................................................................................... 1500/5189
[00:54:06] ................................i................................................................... 1600/5189
[00:54:09] .i.................................................................................................. 1700/5189
[00:54:09] .i.................................................................................................. 1700/5189
[00:54:13] .................................................................................................... 1800/5189
[00:54:16] .................................................................................................... 1900/5189
[00:54:19] ............................................i....................................................... 2000/5189
[00:54:22] ..........................................................................F......................... 2100/5189
[00:54:29] .................................................................................................... 2300/5189
[00:54:33] .................................................................................................... 2400/5189
[00:54:36] .................................................................................................... 2500/5189
[00:54:40] .................................................................................................... 2600/5189
---
[00:56:04] 
[00:56:04] ---- [ui] ui/did_you_mean/recursion_limit.rs stdout ----
[00:56:04] diff of stderr:
[00:56:04] 
[00:56:04] - error[E0275]: overflow evaluating the requirement `K: std::marker::Send`
[00:56:04] + error[E0275]: overflow evaluating the requirement `F: std::marker::Send`
[00:56:04] 3    |
[00:56:04] 3    |
[00:56:04] 4 LL |     is_send::<A>(); //~ ERROR overflow evaluating the requirement
[00:56:04] 5    |     ^^^^^^^^^^^^
[00:56:04] 6    |
[00:56:04] 6    |
[00:56:04] 7    = help: consider adding a `#![recursion_limit="20"]` attribute to your crate
[00:56:04] -    = note: required because it appears within the type `J`
[00:56:04] -    = note: required because it appears within the type `I`
[00:56:04] -    = note: required because it appears within the type `H`
[00:56:04] -    = note: required because it appears within the type `G`
[00:56:04] -    = note: required because it appears within the type `F`
[00:56:04] 13    = note: required because it appears within the type `E`
[00:56:04] 14    = note: required because it appears within the type `D`
[00:56:04] 15    = note: required because it appears within the type `C`
[00:56:04] 
[00:56:04] The actual stderr differed from the expected stderr.
[00:56:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit/recursion_limit.stderr
[00:56:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit/recursion_limit.stderr
[00:56:04] To update references, rerun the tests and pass the `--bless` flag
[00:56:04] To only update this specific test, also pass `--test-args did_you_mean/recursion_limit.rs`
[00:56:04] error: 1 errors occurred comparing output.
[00:56:04] status: exit code: 1
[00:56:04] status: exit code: 1
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/recursion_limit.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux required because it appears within the type `C`\n   = note: required because it appears within the type `B`\n   = note: required because it appears within the type `A`\nnote: required by `is_send`\n  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:41:1\n   |\nLL | fn is_send<T:Send>() { }\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:56:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:56:04] {"message":"For more information about this error, try `rustc --explain E0275`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0275`.\n"}
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[ui] ui/did_you_mean/recursion_limit.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:56:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:04] 
[00:56:04] ---- [ui] ui/error-codes/E0055.rs stdout ----
[00:56:04] diff of stderr:
[00:56:04] 
[00:56:04] - error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
[00:56:04] -   --> $DIR/E0055.rs:21:13
[00:56:04] + error[E0275]: overflow evaluating the requirement `&Foo: std::marker::Unsize<_>`
[00:56:04] +   --> $DIR/E0055.rs:20:19
[00:56:04] 3    |
[00:56:04] - LL |     ref_foo.foo();
[00:56:04] -    |             ^^^ deref recursion limit reached
[00:56:04] + LL |     let ref_foo = &&Foo;
[00:56:04] 6    |
[00:56:04] 6    |
[00:56:04] 7    = help: consider adding a `#![recursion_limit="4"]` attribute to your crate
[00:56:04] +    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<*const _>` for `&&Foo`
[00:56:04] 9 error: aborting due to previous error
[00:56:04] 10 
[00:56:04] 
[00:56:04] - For more information about this error, try `rustc --explain E0055`.
[00:56:04] - For more information about this error, try `rustc --explain E0055`.
[00:56:04] + For more information about this error, try `rustc --explain E0275`.
[00:56:04] 12 
[00:56:04] 
[00:56:04] 
[00:56:04] The actual stderr differed from the expected stderr.
[00:56:04] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0055/E0055.stderr
[00:56:04] To update references, rerun the tests and pass the `--bless` flag
[00:56:04] To only update this specific test, also pass `--test-args error-codes/E0055.rs`
[00:56:04] error: 1 errors occurred comparing output.
[00:56:04] status: exit code: 1
[00:56:04] status: exit code: 1
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0055.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0055/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0055/auxiliary" "-A" "unused"
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] {"message":"overflow evaluating the requirement `&Foo: std::marker::Unsize<_>`","code":{"code":"E0275","explanation":"\nThis error occurs when there was a recursive trait requirement that overflowed\nbefore it could be evaluated. Often this means that there is unbounded\nrecursion in resolving some type bounds.\n\nFor example, in the following code:\n\n