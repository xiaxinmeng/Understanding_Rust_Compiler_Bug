plain
travis_time:end:32bad984:start=1545305130986405252,finish=1545305185874089297,duration=54887684045
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:26] 
[01:04:26] ---- [ui] ui/consts/static_mut_containing_mut_ref2.rs stdout ----
[01:04:26] diff of stderr:
[01:04:26] 
[01:04:26] 4 LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[01:04:26] 5    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[01:04:26] - error: aborting due to previous error
[01:04:26] - error: aborting due to previous error
[01:04:26] + error[E0019]: static contains unimplemented expression type
[01:04:26] +   --> $DIR/static_mut_containing_mut_ref2.rs:5:45
[01:04:26] +    |
[01:04:26] + LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[01:04:26] 8 
[01:04:26] - For more information about this error, try `rustc --explain E0017`.
[01:04:26] + error: aborting due to 2 previous errors
[01:04:26] + 
[01:04:26] + 
[01:04:26] + Some errors occurred: E0017, E0019.
[01:04:26] + For more information about an error, try `rustc --explain E0017`.
[01:04:26] 10 
[01:04:26] 
[01:04:26] 
[01:04:26] The actual stderr differed from the expected stderr.
[01:04:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/static_mut_containing_mut_ref2.stderr
[01:04:26] To update references, rerun the tests and pass the `--bless` flag
[01:04:26] To only update this specific test, also pass `--test-args consts/static_mut_containing_mut_ref2.rs`
[01:04:26] error: 1 errors occurred comparing output.
[01:04:26] status: exit code: 1
[01:04:26] status: exit code: 1
[01:04:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/auxiliary" "-A" "unused"
[01:04:26] ------------------------------------------
[01:04:26] 
[01:04:26] ------------------------------------------
[01:04:26] stderr:
[01:04:26] stderr:
[01:04:26] ------------------------------------------
[01:04:26] {"message":"references in statics may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n