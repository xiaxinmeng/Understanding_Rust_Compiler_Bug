plain
travis_time:end:14b1cb35:start=1558550770923618527,finish=1558550773123272482,duration=2199653955
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:14]      |
[00:08:14] 535  | impl<'a> Parser<'a> {
[00:08:14]      |      -- lifetime `'a` defined here
[00:08:14] ...
[00:08:14] 2335 |     fn parse_bottom_expr(&mut self) -> PResult<'a, P<Expr>> {
[00:08:14] ...
[00:08:14] ...
[00:08:14] 2602 |                             return Err(self.expected_expression_found());
[00:08:14]      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'a`
[00:08:16] error: aborting due to previous error
[00:08:16] 
[00:08:16] error: Could not compile `syntax`.
[00:08:16] 
---
travis_time:end:00dbb994:start=1558551281485425154,finish=1558551281491338304,duration=5913150
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f98c010
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2902a116
travis_time:start:2902a116
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:167fa396
$ dmesg | grep -i kill
