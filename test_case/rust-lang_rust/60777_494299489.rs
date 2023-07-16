plain
travis_time:end:0de53a82:start=1558427365268330498,finish=1558427454604052219,duration=89335721721
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:07e27820
rm 'src/doc/book'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/29fe982990e43b9367be0ff47abc82fb2123fd03.tar.gz
[00:00:00] rm 'src/tools/rustfmt'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-rustfmt.tar.gz &&         curl -f -sSL -o download-src-tools-rustfmt.tar.gz https://github.com/rust-lang-nursery/rustfmt/archive/5274b49caa1a7db6ac10c76bf1a3d5710ccef569.tar.gz
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/811c697b232c611ed754d279ed20643a0c4096f6.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm-emscripten.tar.gz &&         curl -f -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz
[00:00:00] rm 'src/llvm-project'
---

[00:03:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:50] tidy error: /checkout/src/ci/init_repo.sh:49: line longer than 100 chars
[00:03:55] some tidy checks failed
[00:03:55] 
[00:03:55] 
[00:03:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:55] 
[00:03:55] 
[00:03:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:55] Build completed unsuccessfully in 0:01:12
[00:03:55] Build completed unsuccessfully in 0:01:12
[00:03:55] make: *** [tidy] Error 1
[00:03:55] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08652cc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 21 08:34:59 UTC 2019
---
travis_time:end:0276a520:start=1558427700174803815,finish=1558427700179386109,duration=4582294
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:205e0a50
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aa4ea70
travis_time:start:0aa4ea70
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bff0bb0
$ dmesg | grep -i kill
