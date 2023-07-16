plain
    100% |████████████████████████████████| 61kB 7.7MB/s 
Collecting botocore==1.10.62 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/24/ec/95df2edaa21e426581f41745e0de355170b8cb6eed2a2a5641c0c348df33/botocore-1.10.62-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 39.4MB/s eta 0:00:01
    0% |▏                               | 20kB 16.8MB/s eta 0:00:01
    0% |▎                               | 30kB 22.0MB/s eta 0:00:01
    0% |▎                               | 40kB 15.0MB/s eta 0:00:01
---
[01:07:54]    Doc-tests core
[01:08:00] 
[01:08:00] running 2100 tests
[01:08:08] ....................................................................................................
[01:08:16] .........................F...FFF....................................................................
[01:08:36] .....................................i..............................................................
[01:08:43] ....................................................................................................
[01:08:51] ....................................................................................................
[01:08:59] ....................................................................................................
---
[01:10:52] ............................i...............................................i.......................
[01:10:52] 
[01:10:52] failures:
[01:10:52] 
[01:10:52] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 567) stdout ----
[01:10:52] error[E0599]: no method named `unwrap` found for type `&mut [u16]` in the current scope
[01:10:52]  --> char/methods.rs:570:39
[01:10:52]   |
[01:10:52] 6 | let result = '𝕊'.encode_utf16(&mut b).unwrap();
[01:10:52] 
[01:10:52] thread 'char/methods.rs - char::methods::char::try_encode_utf16 (line 567)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:10:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:10:52] 
[01:10:52] 
[01:10:52] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 478) stdout ----
[01:10:52] error[E0599]: no method named `unwrap` found for type `&mut str` in the current scope
[01:10:52]  --> char/methods.rs:481:38
[01:10:52]   |
[01:10:52] 6 | let result = 'ß'.encode_utf8(&mut b).unwrap();
[01:10:52] 
[01:10:52] thread 'char/methods.rs - char::methods::char::try_encode_utf8 (line 478)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:10:52] 
[01:10:52] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 579) stdout ----
[01:10:52] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 579) stdout ----
[01:10:52] error[E0308]: mismatched types
[01:10:52]  --> char/methods.rs:582:1
[01:10:52]   |
[01:10:52] 6 | assert_eq!(None, '𝕊'.encode_utf16(&mut b));
[01:10:52]   | |
[01:10:52]   | |
[01:10:52]   | expected enum `std::option::Option`, found &mut [u16]
[01:10:52]   | help: try using a variant of the expected type: `Some(*right_val)`
[01:10:52]   = note: expected type `std::option::Option<_>`
[01:10:52]              found type `&mut [u16]`
[01:10:52]   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:10:52] 
[01:10:52] 
[01:10:52] thread 'char/methods.rs - char::methods::char::try_encode_utf16 (line 579)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:10:52] 
[01:10:52] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 490) stdout ----
[01:10:52] error[E0308]: mismatched types
[01:10:52]  --> char/methods.rs:493:1
[01:10:52]   |
[01:10:52] 6 | assert_eq!(None, 'ß'.encode_utf8(&mut b));
[01:10:52]   | |
[01:10:52]   | |
[01:10:52]   | expected enum `std::option::Option`, found &mut str
[01:10:52]   | help: try using a variant of the expected type: `Some(*right_val)`
[01:10:52]   = note: expected type `std::option::Option<_>`
[01:10:52]              found type `&mut str`
[01:10:52]   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:10:52] 
---
[01:10:52] 
[01:10:52] error: test failed, to rerun pass '--doc'
[01:10:52] 
[01:10:52] 
[01:10:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:10:52] 
[01:10:52] 
[01:10:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:52] Build completed unsuccessfully in 0:23:06
[01:10:52] Build completed unsuccessfully in 0:23:06
[01:10:52] make: *** [check] Error 1
[01:10:52] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:017def04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
