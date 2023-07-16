plain
travis_time:end:0f770f87:start=1545668103208610855,finish=1545668106004898033,duration=2796287178
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:54:32] .................................................................................................... 2300/5198
[00:54:36] .................................................................................................... 2400/5198
[00:54:39] .................................................................................................... 2500/5198
[00:54:42] .................................................................................................... 2600/5198
[00:54:47] ..........................F......................................................................... 2700/5198
[00:54:53] .................................................................................................... 2900/5198
[00:54:56] .................................................................................................... 3000/5198
[00:54:59] ..................................................................................................i. 3100/5198
[00:55:02] .................................................................................................... 3200/5198
---
[00:56:05] 
[00:56:05] ---- [ui] ui/issues/issue-45157.rs stdout ----
[00:56:05] diff of stderr:
[00:56:05] 
[00:56:05] - error[E0502]: cannot borrow `u.z.c` as immutable because it is also borrowed as mutable
[00:56:05] + error[E0502]: cannot borrow `u.z` (via `u.z.c`) as immutable because it is also borrowed as mutable (via `u.s.a`)
[00:56:05] 3    |
[00:56:05] 3    |
[00:56:05] 4 LL |         let mref = &mut u.s.a;
[00:56:05] -    |                    ---------- mutable borrow occurs here
[00:56:05] -    |                    ---------- mutable borrow occurs here
[00:56:05] +    |                    ---------- mutable borrow occurs here (via `u.s.a`)
[00:56:05] 6 ...
[00:56:05] 7 LL |         let nref = &u.z.c;
[00:56:05] -    |                    ^^^^^^ immutable borrow occurs here
[00:56:05] +    |                    ^^^^^^ immutable borrow occurs here (via `u.z.c`)
[00:56:05] 9 LL |         //~^ ERROR cannot borrow `u.z.c` as immutable because it is also borrowed as mutable [E0502]
[00:56:05] 10 LL |         println!("{} {}", mref, nref)
[00:56:05] 11    |                           ---- mutable borrow later used here
[00:56:05] 
[00:56:05] The actual stderr differed from the expected stderr.
[00:56:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45157/issue-45157.stderr
[00:56:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45157/issue-45157.stderr
[00:56:05] To update references, rerun the tests and pass the `--bless` flag
[00:56:05] To only update this specific test, also pass `--test-args issues/issue-45157.rs`
[00:56:05] error: 1 errors occurred comparing output.
[00:56:05] status: exit code: 1
[00:56:05] status: exit code: 1
[00:56:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45157.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45157/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45157/auxiliary" "-A" "unused"
[00:56:05] ------------------------------------------
[00:56:05] 
[00:56:05] ------------------------------------------
[00:56:05] stderr:
[00:56:05] stderr:
[00:56:05] ------------------------------------------
[00:56:05] {"message":"cannot borrow `u.z` (via `u.z.c`) as immutable because it is also borrowed as mutable (via `u.s.a`)","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n