plain
travis_time:end:3b4135de:start=1548845451376193878,finish=1548845520945359157,duration=69569165279
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:13]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:59:14]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:59:26]     Finished release [optimized] target(s) in 17.60s
[00:59:26] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:59:26] error: Found argument '--generate-redirect-pages' which wasn't expected, or isn't valid in this context
[00:59:26] USAGE:
[00:59:26] USAGE:
[00:59:26]     cargo doc --color <WHEN> --jobs <N> --locked --manifest-path <PATH> --release --target <TRIPLE> -Z <FLAG>...
[00:59:26] For more information try --help
[00:59:26] 
[00:59:26] 
[00:59:26] 
[00:59:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "--generate-redirect-pages" "--no-deps" "-p" "test"
[00:59:26] 
[00:59:26] 
[00:59:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:59:26] Build completed unsuccessfully in 0:05:22
[00:59:26] Build completed unsuccessfully in 0:05:22
[00:59:26] Makefile:18: recipe for target 'all' failed
[00:59:26] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17d70a82
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan 30 11:51:36 UTC 2019
---
travis_time:end:08cc3890:start=1548849097982276465,finish=1548849097988551762,duration=6275297
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:31a25563
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cfa673c
travis_time:start:0cfa673c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:072e2380
$ dmesg | grep -i kill
