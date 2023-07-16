plain
[01:36:29]    Compiling cargo v0.32.0
[01:36:35] error[E0308]: mismatched types
[01:36:35]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cargo-0.32.0/src/cargo/util/sha256.rs:9:34
[01:36:35]   |
[01:36:35] 9 |         let hasher = Hasher::new(Algorithm::SHA256);
[01:36:35]   |                                  |
[01:36:35]   |                                  |
[01:36:35]   |                                  expected reference, found enum `util::sha256::crypto_hash::Algorithm`
[01:36:35]   |                                  help: consider borrowing here: `&Algorithm::SHA256`
[01:36:35]   |
[01:36:35]   = note: expected type `&util::sha256::crypto_hash::Algorithm`
[01:36:35]              found type `util::sha256::crypto_hash::Algorithm`
[01:36:35] error: aborting due to previous error
[01:36:35] 
[01:36:35] For more information about this error, try `rustc --explain E0308`.
[01:36:35] For more information about this error, try `rustc --explain E0308`.
[01:36:35] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[01:36:35] Caused by:
[01:36:35]   Could not compile `cargo`.
[01:36:35] 
[01:36:35] To learn more, run the command again with --verbose.
[01:36:35] To learn more, run the command again with --verbose.
[01:36:35] 
[01:36:35] 
[01:36:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[01:36:35] 
[01:36:35] 
[01:36:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:36:35] Build completed unsuccessfully in 1:30:43
---
travis_time:end:0fd00d0c:start=1545331532947154253,finish=1545331532959197945,duration=12043692
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05e5c405
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0eb80e5e
travis_time:start:0eb80e5e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0df16a48
$ dmesg | grep -i kill
