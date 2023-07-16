plain
[00:04:12]  Downloading quote v0.5.1
[00:04:12]  Downloading unicode-xid v0.1.0
[00:04:12]  Downloading fixedbitset v0.1.9
[00:04:12]  Downloading ordermap v0.3.5
[00:04:28] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/ordermap/0.3.5/download`, got 500
[00:04:47] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/ordermap/0.3.5/download`, got 500
[00:05:04] error: unable to get packages from source
[00:05:04] Caused by:
[00:05:04] Caused by:
[00:05:04]   failed to get 200 response from `https://crates.io/api/v1/crates/ordermap/0.3.5/download`, got 500
[00:05:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
travis_time:end:0ea59dae:start=1529818550034692430,finish=1529818855004647086,duration=304969954656

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:095bcf6b
---
travis_time:end:011976f0:start=1529818855259257381,finish=1529818855265567341,duration=6309960
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2783be26
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:034fc5a8
$ dmesg | grep -i kill
