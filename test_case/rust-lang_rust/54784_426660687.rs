plain
[00:02:01] extracting /checkout/obj/build/cache/2018-09-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:01] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:04]     Updating crates.io index
[00:02:09]     Updating crates.io index
[00:02:10] error: no matching package named `libserde-json` found
[00:02:10] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:02:10] required by package `rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)`
[00:02:10]     ... which is depended on by `rls v0.130.5 (/checkout/src/tools/rls)`
[00:02:10] Build completed unsuccessfully in 0:00:22
[00:02:10] Makefile:81: recipe for target 'prepare' failed
[00:02:10] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:11]     Updating crates.io index
[00:02:11] error: no matching package named `libserde-json` found
[00:02:11] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:02:11] required by package `rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)`
[00:02:11]     ... which is depended on by `rls v0.130.5 (/checkout/src/tools/rls)`
[00:02:11] Build completed unsuccessfully in 0:00:00
[00:02:11] make: *** [prepare] Error 1
[00:02:11] Makefile:81: recipe for target 'prepare' failed
[00:02:13] Command failed. Attempt 3/5:
[00:02:13] Command failed. Attempt 3/5:
[00:02:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:13]     Updating crates.io index
[00:02:13] error: no matching package named `libserde-json` found
[00:02:13] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:02:13] required by package `rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)`
[00:02:13]     ... which is depended on by `rls v0.130.5 (/checkout/src/tools/rls)`
[00:02:13] Build completed unsuccessfully in 0:00:00
[00:02:13] Makefile:81: recipe for target 'prepare' failed
[00:02:13] make: *** [prepare] Error 1
[00:02:16] Command failed. Attempt 4/5:
[00:02:16] Command failed. Attempt 4/5:
[00:02:16] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:17]     Updating crates.io index
[00:02:17] error: no matching package named `libserde-json` found
[00:02:17] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:02:17] required by package `rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)`
[00:02:17]     ... which is depended on by `rls v0.130.5 (/checkout/src/tools/rls)`
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:81: recipe for target 'prepare' failed
[00:02:21] Command failed. Attempt 5/5:
[00:02:21] Command failed. Attempt 5/5:
[00:02:21] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:21]     Updating crates.io index
[00:02:21] error: no matching package named `libserde-json` found
[00:02:21] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:02:21] required by package `rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)`
[00:02:21]     ... which is depended on by `rls v0.130.5 (/checkout/src/tools/rls)`
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:81: recipe for target 'prepare' failed
[00:02:21] The command has failed after 5 attempts.
---
travis_time:end:117edec4:start=1538577267397516502,finish=1538577267403434180,duration=5917678
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11b34958
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:108f1b6c
travis_time:start:108f1b6c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0462873c
$ dmesg | grep -i kill
