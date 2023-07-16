plain
travis_time:end:1be0d33c:start=1555511561041899128,finish=1555511563287386925,duration=2245487797
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:13]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:14] error[E0423]: expected value, found struct `File`
[00:05:14]    --> src/libstd/fs.rs:904:47
[00:05:14]     |
[00:05:14] 904 |         fs_imp::File::open(path, &self.0).map(File)
[00:05:14]     |                                               ^^^^ did you mean `File { /* fields */ }`?
[00:05:14]     |
[00:05:14] 10  | use crate::sys::unix::fs::File;
[00:05:14]     |
[00:05:14] 
---
travis_time:end:06716e32:start=1555511891775138881,finish=1555511891780816641,duration=5677760
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09666163
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f698b96
travis_time:start:0f698b96
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b4918ac
$ dmesg | grep -i kill
