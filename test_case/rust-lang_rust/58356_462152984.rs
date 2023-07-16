plain
travis_time:end:206b93fb:start=1549817124181447245,finish=1549817277234657563,duration=153053210318
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:17]   Downloaded slab v0.4.2
[00:02:17]   Downloaded libz-sys v1.0.25
[00:02:17]   Downloaded rustc-ap-rustc_data_structures v306.0.0
[00:02:17]   Downloaded rls-span v0.4.0
[00:02:17]   Downloaded blake2-rfc v0.2.18
[00:02:17]   Downloaded unicode-bidi v0.3.4
[00:02:17]   Downloaded rand_core v0.2.2
[00:02:17]   Downloaded tokio-codec v0.1.1
[00:02:17]   Downloaded xattr v0.2.2
[00:02:17]   Downloaded xattr v0.2.2
[00:02:17]   Downloaded strsim v0.7.0
[00:02:17]   Downloaded memmap v0.6.2
[00:02:17]   Downloaded string_cache_codegen v0.4.1
[00:02:17]   Downloaded generic-array v0.9.0
[00:02:17]   Downloaded curl-sys v0.4.15
[00:02:17]   Downloaded constant_time_eq v0.1.3
[00:02:17]   Downloaded diff v0.1.11
[00:02:17]   Downloaded open v1.2.1
[00:02:17]   Downloaded remove_dir_all v0.5.1
[00:02:17]   Downloaded globset v0.4.2
---
[00:02:22]   Downloaded fs2 v0.4.3
[00:02:22]   Downloaded url_serde v0.2.0
[00:02:22]   Downloaded mdbook v0.2.3
[00:02:22]   Downloaded openssl-probe v0.1.2
[00:02:22]   Downloaded argon2rs v0.2.5
[00:02:22]   Downloaded rls-vfs v0.7.0
[00:02:22]   Downloaded memchr v2.1.1
[00:02:22]   Downloaded mio v0.6.16
[00:02:22]   Downloaded rustc-ap-serialize v306.0.0
---
tidy check
[00:03:09] * 565 error codes
[00:03:09] * highest error code: E0722
[00:03:09] * 250 features
[00:03:10] crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
[00:03:10]   * rustc-ap-syntax 306.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:03:10]   * rustc-ap-syntax 366.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:03:10] some tidy checks failed
[00:03:10] 
[00:03:10] 
[00:03:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:10] 
[00:03:10] 
[00:03:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:10] Build completed unsuccessfully in 0:00:45
[00:03:10] Build completed unsuccessfully in 0:00:45
[00:03:10] Makefile:68: recipe for target 'tidy' failed
[00:03:10] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:050c2c2c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 16:51:16 UTC 2019
---
travis_time:end:1cbdb471:start=1549817477216153747,finish=1549817477220603282,duration=4449535
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29e080fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3bd73786
travis_time:start:3bd73786
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1355d6d0
$ dmesg | grep -i kill
