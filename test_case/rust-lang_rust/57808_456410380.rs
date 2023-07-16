plain
travis_time:end:0dc0ccf8:start=1548165344292103971,finish=1548165417886748937,duration=73594644966
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:00:00] Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide) registered for path 'src/doc/edition-guide'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/gnzlbg/stdsimd.git) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
---
#####################################################                     74.6%
######################################################################## 100.0%
[00:01:17] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:17]     Updating crates.io index
[00:01:27]     Updating git repository `https://github.com/gnzlbg/stdsimd`
[00:01:29]   Downloaded time v0.1.40
[00:01:29]   Downloaded lazy_static v0.2.11
[00:01:29]   Downloaded toml v0.4.10
[00:01:29]   Downloaded num_cpus v1.8.0
---
travis_time:start:tidy
tidy check
[00:03:25] * 564 error codes
[00:03:25] * highest error code: E0721
[00:03:25] tidy error: /checkout/src/libstd/lib.rs:346: mismatches the `issue` in previous
[00:03:26] Dependencies not on the whitelist:
[00:03:26] * wasm-bindgen 
[00:03:26] * wasm-bindgen-backend 
[00:03:26] * wasm-bindgen-macro 
[00:03:26] * wasm-bindgen-macro-support 
[00:03:26] * wasm-bindgen-shared 
[00:03:26] invalid source: "git+https://github.com/gnzlbg/stdsimd?branch=refactor#13941f86a89aca89acf3bb38c209f2ad2fb972c9"
[00:03:26] some tidy checks failed
[00:03:26] 
[00:03:26] 
[00:03:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:26] 
[00:03:26] 
[00:03:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:26] Build completed unsuccessfully in 0:00:48
[00:03:26] Build completed unsuccessfully in 0:00:48
[00:03:26] make: *** [tidy] Error 1
[00:03:26] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:106074b4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 22 14:00:33 UTC 2019
---
travis_time:end:09862f7d:start=1548165634103576758,finish=1548165634108290597,duration=4713839
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:215c62da
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:242f3b80
travis_time:start:242f3b80
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0525c188
$ dmesg | grep -i kill
