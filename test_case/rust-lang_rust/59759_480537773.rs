plain
travis_time:end:1425fc38:start=1554584592395938786,finish=1554584672686721936,duration=80290783150
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:54]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:03:57] error[E0308]: mismatched types
[00:03:57]    --> src/libstd/io/buffered.rs:529:35
[00:03:57]     |
[00:03:57] 529 |     pub fn get_ref(&self) -> &W { self.inner }
[00:03:57]     |                                   |
[00:03:57]     |                                   |
[00:03:57]     |                                   expected &W, found type parameter
[00:03:57]     |                                   help: consider borrowing here: `&self.inner`
[00:03:57]     = note: expected type `&W`
[00:03:57]                found type `W`
[00:03:57] 
[00:03:57] error[E0308]: mismatched types
[00:03:57] error[E0308]: mismatched types
[00:03:57]    --> src/libstd/io/buffered.rs:547:43
[00:03:57]     |
[00:03:57] 547 |     pub fn get_mut(&mut self) -> &mut W { self.inner }
[00:03:57]     |                                           |
[00:03:57]     |                                           |
[00:03:57]     |                                           expected &mut W, found type parameter
[00:03:57]     |                                           help: consider mutably borrowing here: `&mut self.inner`
[00:03:57]     = note: expected type `&mut W`
[00:03:57]                found type `W`
[00:03:57] 
[00:03:58] error: aborting due to 2 previous errors
---
travis_time:end:03b7b6c4:start=1554584920996306550,finish=1554584921001820522,duration=5513972
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27a0a502
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1cd10ffc
travis_time:start:1cd10ffc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12f70974
