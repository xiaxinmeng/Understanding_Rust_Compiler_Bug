plain
travis_time:end:13c999e6:start=1550077800321475949,finish=1550077803247153220,duration=2925677271
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:07:02]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:07]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:13:16]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:16]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:17] error[E0277]: the trait bound `!: std::convert::From<()>` is not satisfied
[00:13:17]    --> src/librustc_metadata/encoder.rs:105:9
[00:13:17]     |
[00:13:17] 105 |         self.emit_usize(seq.len)?;
[00:13:17]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<()>` is not implemented for `!`
[00:13:17]     = help: the following implementations were found:
[00:13:17]     = help: the following implementations were found:
[00:13:17]               <! as std::convert::From<std::convert::Infallible>>
[00:13:17]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:13:17] 
[00:13:17] 
[00:13:17] error[E0277]: the trait bound `!: std::convert::From<()>` is not satisfied
[00:13:17]    --> src/librustc_metadata/encoder.rs:128:9
[00:13:17]     |
[00:13:17] 128 |         krate.encode(self)?;
[00:13:17]     |         ^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<()>` is not implemented for `!`
[00:13:17]     = help: the following implementations were found:
[00:13:17]     = help: the following implementations were found:
[00:13:17]               <! as std::convert::From<std::convert::Infallible>>
[00:13:17]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:13:17] 
[00:13:17] 
[00:13:17] error[E0277]: the trait bound `!: std::convert::From<()>` is not satisfied
[00:13:17]    --> src/librustc_metadata/encoder.rs:163:9
[00:13:17]     |
[00:13:17] 163 |         TAG_VALID_SPAN.encode(self)?;
[00:13:17]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<()>` is not implemented for `!`
[00:13:17]     = help: the following implementations were found:
[00:13:17]     = help: the following implementations were found:
[00:13:17]               <! as std::convert::From<std::convert::Infallible>>
[00:13:17]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:13:17] 
[00:13:17] 
[00:13:17] error[E0277]: the trait bound `!: std::convert::From<()>` is not satisfied
[00:13:17]    --> src/librustc_metadata/encoder.rs:164:9
[00:13:17]     |
[00:13:17] 164 |         span.lo.encode(self)?;
[00:13:17]     |         ^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<()>` is not implemented for `!`
[00:13:17]     = help: the following implementations were found:
[00:13:17]     = help: the following implementations were found:
[00:13:17]               <! as std::convert::From<std::convert::Infallible>>
[00:13:17]     = note: the trait is implemented for `()`. Possibly this error has been caused by changes to Rust's type-inference algorithm (see: https://github.com/rust-lang/rust/issues/48950 for more info). Consider whether you meant to use the type `()` here instead.
[00:13:17] 
[00:13:18] error: aborting due to 4 previous errors
[00:13:18] 
[00:13:18] For more information about this error, try `rustc --explain E0277`.
[00:13:18] For more information about this error, try `rustc --explain E0277`.
[00:13:18] error: Could not compile `rustc_metadata`.
[00:13:18] warning: build failed, waiting for other jobs to finish...
[00:17:37] error: build failed
[00:17:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:17:37] expected success, got: exit code: 101
[00:17:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:17:37] Build completed unsuccessfully in 0:12:35
[00:17:37] Makefile:18: recipe for target 'all' failed
[00:17:37] make: *** [all] Error 1
69832 ./src/test
69060 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
60756 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
58884 ./.git/modules/src/tools
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:028b1ec2
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/home/travis/Librarers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04593987
$ dmesg | grep -i kill
