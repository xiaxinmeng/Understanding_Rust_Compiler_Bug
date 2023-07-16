plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:03791244
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4f/dd/d1b374af4e5f374173517e7d63e2f4489d3d2e9a45626df830c885412cc9/awscli-1.16.64-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 9.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:06:37]    Compiling chalk-engine v0.8.0
[00:06:37]    Compiling env_logger v0.5.12
[00:06:37]    Compiling tempfile v3.0.3
[00:06:40]    Compiling parking_lot_core v0.3.0
[00:06:40]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:06:42]    Compiling crossbeam-epoch v0.3.1
[00:06:42]    Compiling log_settings v0.1.2
[00:06:45]    Compiling parking_lot v0.6.4
[00:06:45]    Compiling crossbeam-deque v0.2.0
---
[00:54:05]    Compiling chalk-engine v0.8.0
[00:54:05]    Compiling env_logger v0.5.12
[00:54:06]    Compiling tempfile v3.0.3
[00:54:08]    Compiling parking_lot_core v0.3.0
[00:54:08]    Compiling rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[00:54:10]    Compiling crossbeam-epoch v0.3.1
[00:54:11]    Compiling log_settings v0.1.2
[00:54:12]    Compiling parking_lot v0.6.4
[00:54:12]    Compiling flate2 v1.0.3
---
[01:26:13]     Checking smallvec v0.6.5
[01:26:14]     Checking lazy_static v1.1.0
[01:26:14]     Checking lock_api v0.1.3
[01:26:14]     Checking crossbeam-epoch v0.3.1
[01:26:14]     Checking rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[01:26:15]     Checking rustc-rayon-core v0.1.1
[01:26:15]     Checking parking_lot_core v0.3.0
[01:26:15]     Checking parking_lot v0.6.4
[01:26:16]     Checking rustc-rayon v0.1.1
---
[01:26:45]     Checking chalk-engine v0.8.0
[01:26:45]     Checking env_logger v0.5.12
[01:26:46]     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[01:26:47]  Documenting rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
[01:26:47]  Documenting rustc_ezilaires v0.0.0 (/checkout/src/librustc_ezilaires)
[01:26:48]     Checking backtrace-sys v0.1.24
[01:26:48]     Checking jemalloc-sys v0.1.8
[01:26:49]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:26:49]     Checking log_settings v0.1.2
[01:26:49]     Checking log_settings v0.1.2
[01:26:49] warning: `[3]` cannot be resolved, ignoring it...
[01:26:49]     --> src/librustc_ezilaires/json.rs:1275:25
[01:26:49] 1275 | /// For example foo.bar[3].x
[01:26:49]      |                         ^ cannot be resolved, ignoring
[01:26:49]      |
[01:26:49]      = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:26:49]      = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:26:49]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:26:49] 
[01:26:49] warning: `[3]` cannot be resolved, ignoring it...
[01:26:49]     --> src/librustc_ezilaires/json.rs:1284:65
[01:26:49] 1284 | /// StackElements compositing the stack that represents foo.bar[3].x
[01:26:49]      |                                                                 ^ cannot be resolved, ignoring
[01:26:49]      |
[01:26:49]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
---
[01:30:15] travis_fold:start:stage2-error_index_generator
travis_time:start:stage2-error_index_generator
Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
[01:30:15]    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
[01:30:16] error: unused import: `rustc_ezilaires as rustc_serialize`
[01:30:16]    |
[01:30:16]    |
[01:30:16] 16 | extern crate rustc_ezilaires; use rustc_ezilaires as rustc_serialize;
[01:30:16]    |
[01:30:16]    = note: `-D unused-imports` implied by `-D warnings`
[01:30:16] 
[01:30:16] error: aborting due to previous error
---
travis_time:end:080900d0:start=1543445799976665403,finish=1543445799987568599,duration=10903196
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ab0b2d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2aeab7a9
travis_time:start:2aeab7a9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:019cc10d
$ dmesg | grep -i kill
