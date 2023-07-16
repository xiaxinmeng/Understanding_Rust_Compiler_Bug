plain
[00:03:48]    Compiling cargo v0.32.0
[00:03:55] error[E0308]: mismatched types
[00:03:55]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cargo-0.32.0/src/cargo/util/sha256.rs:9:34
[00:03:55]   |
[00:03:55] 9 |         let hasher = Hasher::new(Algorithm::SHA256);
[00:03:55]   |                                  |
[00:03:55]   |                                  |
[00:03:55]   |                                  expected reference, found enum `util::sha256::crypto_hash::Algorithm`
[00:03:55]   |                                  help: consider borrowing here: `&Algorithm::SHA256`
[00:03:55]   |
[00:03:55]   = note: expected type `&util::sha256::crypto_hash::Algorithm`
[00:03:55]              found type `util::sha256::crypto_hash::Algorithm`
[00:03:55] error: aborting due to previous error
[00:03:55] 
[00:03:55] For more information about this error, try `rustc --explain E0308`.
[00:03:55] For more information about this error, try `rustc --explain E0308`.
[00:03:55] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[00:03:55] Caused by:
[00:03:55]   Could not compile `cargo`.
[00:03:55] 
[00:03:55] To learn more, run the command again with --verbose.
[00:03:55] To learn more, run the command again with --verbose.
[00:03:55] 
[00:03:55] 
[00:03:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[00:03:55] 
[00:03:55] 
[00:03:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:03:55] Build completed unsuccessfully in 0:01:35
---
travis_time:end:15e0dd20:start=1545333884866864940,finish=1545333884876299610,duration=9434670
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ebe5000
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0783ca2f
travis_time:start:0783ca2f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c7e3929
$ dmesg | grep -i kill
