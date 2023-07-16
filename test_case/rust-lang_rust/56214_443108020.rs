plain
[01:50:08]     Finished dev [unoptimized + debuginfo] target(s) in 2m 59s
[01:50:08]   Installing /cargo/bin/cargo-vendor
[01:50:08] warning: be sure to add `/cargo/bin` to your PATH to be able to run the installed binaries
[01:50:12]  Downloading openssl-src v111.0.1+1.1.1
[01:50:52] warning: spurious network error (2 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:51:32] warning: spurious network error (1 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:52:18]  Downloading openssl-src v111.0.1+1.1.1
[01:52:58] warning: spurious network error (2 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:53:38] warning: spurious network error (1 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:54:18] 
[01:54:18] Caused by:
[01:54:18]   failed to fetch package
[01:54:18] 
[01:54:18] 
[01:54:18] Caused by:
[01:54:18]   unable to get packages from source
[01:54:18] 
[01:54:18] Caused by:
[01:54:18]   failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:54:18] Caused by:
[01:54:18]   [28] Timeout was reached
[01:54:18] 
[01:54:18] 
---
travis_time:end:06dbf69a:start=1543560158822313955,finish=1543560158832006520,duration=9692565
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00da24a3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0195f6a4
travis_time:start:0195f6a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b8591e
$ dmesg | grep -i kill
