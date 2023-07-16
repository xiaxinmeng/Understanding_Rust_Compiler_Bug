plain
[00:04:18]    Compiling cargo v0.32.0
[00:04:24] error[E0308]: mismatched types
[00:04:24]  --> /cargo/registry/src/github.com-1ecc6299db9ec823/cargo-0.32.0/src/cargo/util/sha256.rs:9:34
[00:04:24]   |
[00:04:24] 9 |         let hasher = Hasher::new(Algorithm::SHA256);
[00:04:24]   |                                  |
[00:04:24]   |                                  |
[00:04:24]   |                                  expected reference, found enum `util::sha256::crypto_hash::Algorithm`
[00:04:24]   |                                  help: consider borrowing here: `&Algorithm::SHA256`
[00:04:24]   |
[00:04:24]   = note: expected type `&util::sha256::crypto_hash::Algorithm`
[00:04:24]              found type `util::sha256::crypto_hash::Algorithm`
[00:04:24] error: aborting due to previous error
[00:04:24] 
[00:04:24] For more information about this error, try `rustc --explain E0308`.
[00:04:24] For more information about this error, try `rustc --explain E0308`.
[00:04:24] error: failed to compile `cargo-vendor v0.1.22`, intermediate artifacts can be found at `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools`
[00:04:24] Caused by:
[00:04:24]   Could not compile `cargo`.
[00:04:24] 
[00:04:24] To learn more, run the command again with --verbose.
[00:04:24] To learn more, run the command again with --verbose.
[00:04:24] 
[00:04:24] 
[00:04:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "-j" "4" "--locked" "--color" "always" "--force" "--debug" "--vers" "0.1.22" "cargo-vendor"
[00:04:24] 
[00:04:24] 
[00:04:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:04:24] Build completed unsuccessfully in 0:01:34
---
travis_time:end:00ed9c89:start=1545332041954381595,finish=1545332041966235229,duration=11853634
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06dbfabc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05d6a5dd
$ dmesg | grep -i kill
