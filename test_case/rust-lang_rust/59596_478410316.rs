plain
travis_time:end:180ab87c:start=1554079212009955415,finish=1554079286697164364,duration=74687208949
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:16:54] diff of stderr:
[01:16:54] 
[01:16:54] 26   --> $DIR/ub-enum.rs:38:1
[01:16:54] 27    |
[01:16:54] 28 LL | const BAD_ENUM4: Wrap<Enum2> = unsafe { TransmuteEnum2 { in2: &0 }.out2 };
[01:16:54] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected something that cannot possibly fail to be in the range 2..=2
[01:16:54] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected something that cannot possibly fail to be in the range 2"..="2
[01:16:54] 30    |
[01:16:54] 31    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[01:16:54] 
[01:16:54] 
[01:16:54] The actual stderr differed from the expected stderr.
[01:16:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.stderr
[01:16:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.stderr
[01:16:54] To update references, rerun the tests and pass the `--bless` flag
[01:16:54] To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`
[01:16:54] error: 1 errors occurred comparing output.
[01:16:54] status: exit code: 1
[01:16:54] status: exit code: 1
[01:16:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary" "-A" "unused"
[01:16:54] ------------------------------------------
[01:16:54] 
[01:16:54] ------------------------------------------
[01:16:54] stderr:
[01:16:54] stderr:
[01:16:54] ------------------------------------------
[01:16:54] {"message":"it is undefined behavior to use this value","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n