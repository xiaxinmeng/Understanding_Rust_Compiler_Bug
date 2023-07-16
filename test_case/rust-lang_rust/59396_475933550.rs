plain
travis_time:end:01b10397:start=1553403288331944254,finish=1553403289282570440,duration=950626186
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:59:54] Verifying status of rustfmt...
[01:59:54] Verifying status of clippy-driver...
[01:59:54] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:59:54] 
[01:59:54] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:59:54] 
[01:59:54] If you do intend to update 'clippy-driver', please check the error messages above and
[01:59:54] commit another update.
307368 ./obj/build/bootstrap/debug/deps
283312 ./src/llvm-project/llvm/test
271528 ./obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target
270748 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
---
155916 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
153076 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
143724 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/release
142512 ./obj/build/bootstrap/debug/incremental/bootstrap-pz4opbgkzvy
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-pz4opbgkzvy/s-famvoi1zxg-968hsz-3u618d595ya6l
122772 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps
108532 ./src/llvm-project/lldb
98628 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
98516 ./obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86
---
travis_time:end:0bf50862:start=1553410495188586063,finish=1553410495193392809,duration=4806746
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10e862e9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11c98cfe
travis_time:start:11c98cfe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

