plain
[00:03:47]    Compiling cargo v0.32.0
[00:03:54] error[E0308]: mismatched types
[00:03:54]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cargo-0.32.0/src/cargo/util/sha256.rs:9:34
[00:03:54]   |
[00:03:54] 9 |         let hasher = Hasher::new(Algorithm::SHA256);
[00:03:54]   |                                  |
[00:03:54]   |                                  |
[00:03:54]   |                                  expected reference, found enum `util::sha256::crypto_hash::Algorithm`
[00:03:54]   |                                  help: consider borrowing here: `&Algorithm::SHA256`
[00:03:54]   |
[00:03:54]   = note: expected type `&util::sha256::crypto_hash::Algorithm`
[00:03:54]              found type `util::sha256::crypto_hash::Algorithm`
[00:03:54] error: aborting due to previous error
[00:03:54] 
[00:03:54] For more information about this error, try `rustc --explain E0308`.
[00:03:54] For more information about this error, try `rustc --explain E0308`.
[00:03:54] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[00:03:54] Caused by:
[00:03:54]   Could not compile `cargo`.
[00:03:54] 
[00:03:54] To learn more, run the command again with --verbose.
[00:03:54] To learn more, run the command again with --verbose.
[00:03:54] 
[00:03:54] 
[00:03:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[00:03:54] 
[00:03:54] 
[00:03:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:03:54] Build completed unsuccessfully in 0:01:35
---
travis_time:end:14329584:start=1545332461225148456,finish=1545332461233051499,duration=7903043
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24badd88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0662e327
travis_time:start:0662e327
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0462256e
$ dmesg | grep -i kill
