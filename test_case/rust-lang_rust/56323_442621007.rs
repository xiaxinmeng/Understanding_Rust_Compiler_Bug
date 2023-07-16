plain
travis_time:end:00549bc9:start=1543439891846548890,finish=1543439894100395132,duration=2253846242
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4f/dd/d1b374af4e5f374173517e7d63e2f4489d3d2e9a45626df830c885412cc9/awscli-1.16.64-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 9.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:04:35]    Compiling polonius-engine v0.5.0
[00:04:35]    Compiling chalk-engine v0.8.0
[00:04:36]    Compiling env_logger v0.5.12
[00:04:36]    Compiling parking_lot_core v0.3.0
[00:04:37]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:04:40]    Compiling parking_lot v0.6.4
[00:04:40]    Compiling crossbeam-epoch v0.3.1
[00:04:42]    Compiling log_settings v0.1.2
[00:04:42]    Compiling flate2 v1.0.3
---
[00:19:18]    Compiling chalk-engine v0.8.0
[00:19:18]    Compiling env_logger v0.5.12
[00:19:18]    Compiling tempfile v3.0.3
[00:19:20]    Compiling parking_lot_core v0.3.0
[00:19:21]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:19:22]    Compiling crossbeam-epoch v0.3.1
[00:19:24]    Compiling log_settings v0.1.2
[00:19:24]    Compiling parking_lot v0.6.4
[00:19:25]    Compiling flate2 v1.0.3
---
[00:40:58]     Checking rand v0.5.5
[00:40:58]     Checking rand v0.4.3
[00:40:58]     Checking atty v0.2.11
[00:40:58]     Checking crossbeam-epoch v0.3.1
[00:40:58]     Checking rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:41:00]     Checking rustc-rayon-core v0.1.1
[00:41:00]     Checking parking_lot_core v0.3.0
[00:41:01]     Checking rustc-rayon v0.1.1
[00:41:01]     Checking parking_lot v0.6.4
[00:41:01]     Checking parking_lot v0.6.4
[00:41:04]     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:41:06]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:41:06]     Checking rustc_target v0.0.0 (/checkout/src/librustc_target)
[00:41:07]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:41:08]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:41:10]     Checking syntax v0.0.0 (/checkout/src/libsyntax)
[00:41:27]     Finished release [optimized] target(s) in 30.70s
[00:41:27] error: unused import: `rustc_ezilaires as rustc_serialize`
[00:41:27]    |
[00:41:27]    |
[00:41:27] 16 | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;
[00:41:27]    |
[00:41:27]    = note: `-D unused-imports` implied by `-D warnings`
[00:41:27] 
[00:41:28] error: aborting due to previous error
---
[00:41:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/error_index_generator/Cargo.toml" "--message-format" "json"
[00:41:28] expected success, got: exit code: 101
[00:41:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:41:28] Build completed unsuccessfully in 0:05:23
[00:41:28] make: *** [all] Error 1
[00:41:28] Makefile:28: recipe for target 'all' failed
2196612 ./obj
2196572 ./obj/build
1550232 ./obj/build/x86_64-unknown-linux-gnu
1161792 ./src
---
150252 ./obj/build/bootstrap/debug/incremental
144028 ./.git
139288 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134656 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f73qjv7g3b-dq619q-18zp7vy2v2s65
133512 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130632 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
128556 ./.git/modules
128552 ./.git/modules/src
---
travis_time:end:02941a9f:start=1543442391953510427,finish=1543442391958987627,duration=5477200
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3768eb8c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|objtravis_time:start:347bd4ae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27cfa735
$ dmesg | grep -i kill
