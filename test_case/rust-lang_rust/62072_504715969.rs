plain
travis_time:end:109271c0:start=1561257967893005173,finish=1561258056360648975,duration=88467643802
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
156504 ./src/llvm-project/clang
150156 ./obj/build/bootstrap/debug/incremental
139060 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
134688 ./obj/build/bootstrap/debug/incremental/bootstrap-1llt3ypt1ftzv
134684 ./obj/build/bootstrap/debug/incremental/bootstrap-1llt3ypt1ftzv/s-fdf440a4ct-b07hyh-v8x3j8rz512o
108532 ./src/llvm-project/lldb
98280 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98116 ./obj/build/-bootstrap-tools/x86_64-unknown-linux-gnu/release
49164 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:239b4acc:start=1561260019919686676,finish=1561260019925347400,duration=5660724
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0071d813
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a34be9c
travis_time:start:0a34be9c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00074dfb
$ dmesg | grep -i kill
