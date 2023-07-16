plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/46/ad/28647c5e1f4bb4094af886e203cfde5543fafd6a5bf830a85909d2058f9f/awscli-1.15.42-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[01:27:15] 
[01:27:15] running 930 tests
[01:27:52] iii.................................................................................................
[01:28:12] ................................................................................................iii.
[01:28:25] .....i......i...i.F....i............................................................................
[01:28:53] ....................iiii........ii..................................................................
[01:29:02] ....................................................................................................
[01:29:20] ...................................................................iiii.............................
[01:29:47] ....................................................................................................
[01:29:47] ....................................................................................................
[01:29:58] ...............................................................................iiii.................
[01:30:03] ..............................
[01:30:03] failures:
[01:30:03] 
[01:30:03] ---- ffi/c_str.rs - ffi::c_str::CString::into_boxed_c_str (line 592) stdout ----
[01:30:03] error: expected one of `.`, `;`, `?`, or an operator, found `::`
[01:30:03]  --> ffi/c_str.rs:595:50
[01:30:03]   |
[01:30:03] 6 | let c_string = CString::new(b"foo".to_vec()).CStr::from_bytes_with_nul;
[01:30:03]   |                                                  ^^ expected one of `.`, `;`, `?`, or an operator here
[01:30:03] 
[01:30:03] error: unused imports: `CStr`, `CString`
[01:30:03]  --> ffi/c_str.rs:593:16
[01:30:03]   |
[01:30:03] 4 | use std::ffi::{CString, CStr};
[01:30:03]   |                ^^^^^^^  ^^^^
[01:30:03] note: lint level defined here
[01:30:03]  --> ffi/c_str.rs:590:9
[01:30:03]   |
[01:30:03] 1 | #![deny(warnings)]
---
[01:30:03] 
[01:30:03] error: test failed, to rerun pass '--doc'
[01:30:03] 
[01:30:03] 
[01:30:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:30:03] 
[01:30:03] 
[01:30:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:03] Build completed unsuccessfully in 0:40:13
[01:30:03] Build completed unsuccessfully in 0:40:13
[01:30:03] make: *** [check] Error 1
[01:30:03] Makefile:58: recipe for target 'check' failed
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e1959a6
travis_time:start:2e1959a6
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11c859b4
$ dmesg | grep -i kill
