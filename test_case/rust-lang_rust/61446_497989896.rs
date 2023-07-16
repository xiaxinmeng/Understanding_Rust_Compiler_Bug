plain
travis_time:end:0ded6a7c:start=1559434517722408498,finish=1559434518583188679,duration=860780181
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:57:10] .................................................................................................... 3300/5610
[00:57:13] ..............................i..................................................................... 3400/5610
[00:57:17] .................................................................................................... 3500/5610
[00:57:20] ....ii...i..ii...................................................................................... 3600/5610
[00:57:24] ..........................................................................F......................... 3700/5610
[00:57:31] ..........ii........................................................................................ 3900/5610
[00:57:33] ...............................i.................................................................... 4000/5610
[00:57:35] ...............................................................................................i.... 4100/5610
[00:57:37] .................................................................................................... 4200/5610
---
[00:58:42] 
[00:58:42] 
[00:58:42] The actual stderr differed from the expected stderr.
[00:58:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/issue-61424.stderr
[00:58:42] To update references, rerun the tests and pass the `--bless` flag
[00:58:42] To only update this specific test, also pass `--test-args nll/issue-61424.rs`
[00:58:42] error: 1 errors occurred comparing output.
[00:58:42] status: exit code: 1
[00:58:42] status: exit code: 1
[00:58:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-61424.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/auxiliary" "-A" "unused"
[00:58:42] ------------------------------------------
[00:58:42] 
[00:58:42] ------------------------------------------
[00:58:42] stderr:
---
travis_time:end:001ae807:start=1559438053309705563,finish=1559438053314812103,duration=5106540
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02089b5a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ec0357e
travis_time:start:2ec0357e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1195b50b
$ dmesg | grep -i kill
