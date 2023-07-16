plain
travis_time:end:037db6ad:start=1544267839193080118,finish=1544267840217569906,duration=1024489788
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:02]   Downloaded smallvec v0.6.5
[00:02:02]   Downloaded hex v0.3.2
[00:02:02]   Downloaded rustc-rayon-core v0.1.1
[00:02:02]   Downloaded rls-vfs v0.7.0
[00:02:02]   Downloaded miniz_oxide v0.2.0
[00:02:02]   Downloaded if_chain v0.1.3
[00:02:02]   Downloaded rayon v1.0.1
[00:02:02]   Downloaded memchr v2.1.1
[00:02:02]   Downloaded rustc-ap-rustc_target v297.0.0
---
[00:02:07]   Downloaded commoncrypto-sys v0.2.0
[00:02:07]   Downloaded redox_syscall v0.1.43
[00:02:07]   Downloaded typenum v1.10.0
[00:02:07]   Downloaded cargo_metadata v0.6.2
[00:02:07]   Downloaded miniz_oxide_c_api v0.2.0
[00:02:07]   Downloaded opener v0.3.2
[00:02:07]   Downloaded diff v0.1.11
[00:02:07]   Downloaded elasticlunr-rs v2.3.3
[00:02:07]   Downloaded crc32fast v1.1.2
---
[00:03:05] * 242 features
[00:03:05] Dependencies not on the whitelist:
[00:03:05] * adler32 
[00:03:05] * build_const 
[00:03:05] * crc 
[00:03:05] * crc32fast 
[00:03:05] * miniz_oxide 
[00:03:05] * miniz_oxide_c_api 
[00:03:05] * rand_hc 
[00:03:05] * rand_isaac 
[00:03:05] * rand_pcg 
[00:03:05] * rand_xorshift 
[00:03:05] * rand_xorshift 
[00:03:05] * rustc_version 
[00:03:05] * semver 
[00:03:05] * semver-parser 
[00:03:05] * serde 
[00:03:05] some tidy checks failed
[00:03:05] 
[00:03:05] 
[00:03:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:05] 
[00:03:05] 
[00:03:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:05] Build completed unsuccessfully in 0:00:56
[00:03:05] Build completed unsuccessfully in 0:00:56
[00:03:05] Makefile:79: recipe for target 'tidy' failed
[00:03:05] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d879064
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 11:20:34 UTC 2018
---
travis_time:end:2c586930:start=1544268034894447539,finish=1544268034900822507,duration=6374968
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d836c87
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:32894360
travis_time:start:32894360
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f768f24
$ dmesg | grep -i kill
