plain
#######################                                                   32.1%
######################################################################## 100.0%
[00:01:16] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:16]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:38] error: no matching package named `unicode_width` found
[00:01:38] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:01:38] did you mean: unicode-width
[00:01:38] required by package `rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)`
[00:01:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:38] make: *** [prepare] Error 1
[00:01:38] Makefile:81: recipe for target 'prepare' failed
[00:01:39] Command failed. Attempt 2/5:
[00:01:39]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:39]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:40] error: no matching package named `unicode_width` found
[00:01:40] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:01:40] did you mean: unicode-width
[00:01:40] required by package `rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)`
[00:01:40] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:40] make: *** [prepare] Error 1
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:42] Command failed. Attempt 3/5:
[00:01:42]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:42]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:42] error: no matching package named `unicode_width` found
[00:01:42] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:01:42] did you mean: unicode-width
[00:01:42] required by package `rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)`
[00:01:42] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:42] make: *** [prepare] Error 1
[00:01:42] Makefile:81: recipe for target 'prepare' failed
[00:01:45] Command failed. Attempt 4/5:
[00:01:45]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:45]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:45] error: no matching package named `unicode_width` found
[00:01:45] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:01:45] did you mean: unicode-width
[00:01:45] required by package `rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)`
[00:01:45] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:45] make: *** [prepare] Error 1
[00:01:45] Makefile:81: recipe for target 'prepare' failed
[00:01:49] Command failed. Attempt 5/5:
[00:01:50]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:50]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:50] error: no matching package named `unicode_width` found
[00:01:50] location searched: registry `https://github.com/rust-lang/crates.io-index`
[00:01:50] did you mean: unicode-width
[00:01:50] required by package `rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)`
[00:01:50] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:50] make: *** [prepare] Error 1
[00:01:50] Makefile:81: recipe for target 'prepare' failed
[00:01:50] The command has failed after 5 attempts.
travis_time:end:05f1a87e:start=1533425775832917228,finish=1533425899080123693,duration=123247206465
---
travis_time:end:0c376752:start=1533425899363227806,finish=1533425899369386542,duration=6158736
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09182ab4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02ee3d68
travis_time:start:02ee3d68
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0de90519
$ dmesg | grep -i kill
