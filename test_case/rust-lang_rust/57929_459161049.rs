plain
travis_time:end:00f1e9e9:start=1548889410316757606,finish=1548889480512194772,duration=70195437166
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:10]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:58:10]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:58:21]     Finished release [optimized] target(s) in 16.74s
[00:58:21] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:58:21] error: Found argument '--no-deps' which wasn't expected, or isn't valid in this context
[00:58:21]  Did you mean --no-default-features?
[00:58:21] USAGE:
[00:58:21] USAGE:
[00:58:21]     cargo rustdoc --color <WHEN> --jobs <N> --locked --manifest-path <PATH> --no-default-features --release --target <TRIPLE>
[00:58:21] For more information try --help
[00:58:21] 
[00:58:21] 
[00:58:21] 
[00:58:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test" "--" "-Z" "unstable-options" "--generate-redirect-pages"
[00:58:21] 
[00:58:21] 
[00:58:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:58:21] Build completed unsuccessfully in 0:05:53
[00:58:21] Build completed unsuccessfully in 0:05:53
[00:58:21] Makefile:18: recipe for target 'all' failed
[00:58:21] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:000c11ba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 00:03:11 UTC 2019
---
travis_time:end:0618df37:start=1548892992660187458,finish=1548892992665774158,duration=5586700
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10e47f39
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03279194
travis_time:start:03279194
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00740f05
$ dmesg | grep -i kill
