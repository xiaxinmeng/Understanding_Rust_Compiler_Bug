plain
travis_time:end:0c1771d4:start=1542631860797261003,finish=1542631862372432762,duration=1575171759
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:09] .............................i...................................................................... 600/5041
[00:52:12] .................................................................................................... 700/5041
[00:52:19] .......................................................................................i...........i 800/5041
[00:52:22] .................................................................................................... 900/5041
[00:52:26] ......iiiii......................................................................................... 1000/5041
[00:52:28] .......................F............................................................................ 1100/5041
[00:52:31] .................................................F.................................................. 1200/5041
[00:52:36] .................................................................................................... 1400/5041
[00:52:38] .................................................................................................... 1500/5041
[00:52:41] .......i....................................................................i....................... 1600/5041
[00:52:45] .................................................................................................... 1700/5041
---
[00:54:27] ..........i......................................................................................... 4700/5041
[00:54:31] .................................................................................................... 4800/5041
[00:54:34] .................................................................................................... 4900/5041
[00:54:36] ................................................................................i................... 5000/5041
[00:54:37] .................................F.......
[00:54:37] 
[00:54:37] ---- [ui] ui/error-codes/E0017.rs stdout ----
[00:54:37] diff of stderr:
[00:54:37] 
[00:54:37] 
[00:54:37] 4 LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
[00:54:37] 5    |                              ^^^^^^ constants require immutable values
[00:54:37] 6 
[00:54:37] + error: cannot mutate statics in the initializer of another static
[00:54:37] +   --> $DIR/E0017.rs:15:39
[00:54:37] +    |
[00:54:37] + LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[00:54:37] + 
[00:54:37] + 
[00:54:37] 7 error[E0017]: references in statics may only refer to immutable values
[00:54:37] 8   --> $DIR/E0017.rs:15:39
[00:54:37] 
[00:54:37] 
[00:54:37] 22 LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
[00:54:37] 23    |                                      ^^^^^^ statics require immutable values
[00:54:37] - error: aborting due to 4 previous errors
[00:54:37] + error: aborting due to 5 previous errors
[00:54:37] 26 
[00:54:37] 27 Some errors occurred: E0017, E0596.
[00:54:37] 27 Some errors occurred: E0017, E0596.
[00:54:37] 28 For more information about an error, try `rustc --explain E0017`.
[00:54:37] 
[00:54:37] 
[00:54:37] The actual stderr differed from the expected stderr.
[00:54:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/E0017.stderr
[00:54:37] To update references, rerun the tests and pass the `--bless` flag
[00:54:37] To only update this specific test, also pass `--test-args error-codes/E0017.rs`
[00:54:37] error: 1 errors occurred comparing output.
[00:54:37] status: exit code: 1
[00:54:37] status: exit code: 1
[00:54:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0017.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/auxiliary" "-A" "unused"
[00:54:37] ------------------------------------------
[00:54:37] 
[00:54:37] ------------------------------------------
[00:54:37] stderr:
[00:54:37] stderr:
[00:54:37] ------------------------------------------
[00:54:37] {"message":"references in constants may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n