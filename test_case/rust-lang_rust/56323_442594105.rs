plain
travis_time:end:07c74040:start=1543435869239933400,finish=1543435870454932377,duration=1214998977
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
    0% |▎                               | 10kB 12.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    2% |█                               | 40kB 1.9MB/s eta 0:00:01
---
[00:04:21]    Compiling chalk-engine v0.8.0
[00:04:21]    Compiling env_logger v0.5.12
[00:04:22]    Compiling tempfile v3.0.3
[00:04:23]    Compiling parking_lot_core v0.3.0
[00:04:23]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:04:26]    Compiling parking_lot v0.6.4
[00:04:26]    Compiling crossbeam-epoch v0.3.1
[00:04:28]    Compiling log_settings v0.1.2
[00:04:28]    Compiling backtrace v0.3.9
---
[00:19:52]    Compiling polonius-engine v0.5.0
[00:19:52]    Compiling chalk-engine v0.8.0
[00:19:52]    Compiling env_logger v0.5.12
[00:19:54]    Compiling parking_lot_core v0.3.0
[00:19:54]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:19:56]    Compiling crossbeam-epoch v0.3.1
[00:19:58]    Compiling log_settings v0.1.2
[00:19:58]    Compiling parking_lot v0.6.4
[00:19:59]    Compiling flate2 v1.0.3
[00:19:59]    Compiling flate2 v1.0.3
[00:20:00]    Compiling backtrace v0.3.9
[00:20:00]    Compiling crossbeam-deque v0.2.0
[00:20:03]    Compiling rustc-rayon v0.1.1
[00:20:07]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:20:07] error: unused import: `rustc_ezilaires as rustc_serialize`
[00:20:07]    |
[00:20:07]    |
[00:20:07] 40 | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize; // used by deriving
[00:20:07]    |
[00:20:07]    = note: `-D unused-imports` implied by `-D warnings`
[00:20:07] 
[00:20:08] error: aborting due to previous error
---
[00:20:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:20:08] expected success, got: exit code: 101
[00:20:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:08] Build completed unsuccessfully in 0:17:12
[00:20:08] make: *** [all] Error 1
[00:20:08] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03bb7c9a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 28 20:31:28 UTC 2018
---
travis_time:end:13402754:start=1543437088655760247,finish=1543437088662219984,duration=6459737
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cdd323e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fo
