plain
travis_time:end:070b0336:start=1558505935735737645,finish=1558505938030056775,duration=2294319130
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:23]      |
[00:08:23] 535  | impl<'a> Parser<'a> {
[00:08:23]      |      -- lifetime `'a` defined here
[00:08:23] ...
[00:08:23] 2335 |     fn parse_bottom_expr(&mut self) -> PResult<'a, P<Expr>> {
[00:08:23] ...
[00:08:23] ...
[00:08:23] 2602 |                             return Err(self.expected_expression_found());
[00:08:23]      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'a`
[00:08:25] error: aborting due to previous error
[00:08:25] 
[00:08:25] error: Could not compile `syntax`.
[00:08:25] 
---
travis_time:end:0365bda6:start=1558506455752251161,finish=1558506455757327863,duration=5076702
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005eecf2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a040a29
travis_time:start:2a040a29
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a6cb51
$ dmesg | grep -i kill
