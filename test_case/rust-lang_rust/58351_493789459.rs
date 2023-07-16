plain
travis_time:end:035eabef:start=1558296993375008689,finish=1558297079302127387,duration=85927118698
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:02]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:13] error[E0061]: this function takes 3 parameters but 4 parameters were supplied
[00:14:13]    --> src/librustc_mir/transform/const_prop.rs:518:18
[00:14:13]     |
[00:14:13] 518 |           self.ecx.validate_operand(
[00:14:13]     | 
[00:14:13]    ::: src/librustc_mir/interpret/validity.rs:640:5
[00:14:13]     |
[00:14:13] 640 | /     pub fn validate_operand(
[00:14:13] 640 | /     pub fn validate_operand(
[00:14:13] 641 | |         &self,
[00:14:13] 642 | |         op: OpTy<'tcx, M::PointerTag>,
[00:14:13] 643 | |         path: Vec<PathElem>,
[00:14:13] 659 | |         visitor.visit_value(op)
[00:14:13] 660 | |     }
[00:14:13]     | |_____- defined here
[00:14:13] 
---
travis_time:end:17e01630:start=1558298109648329458,finish=1558298109653015132,duration=4685674
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:32551be9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3217c506
travis_time:start:3217c506
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18c137d3
$ dmesg | grep -i kill
