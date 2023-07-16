plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/12/92bd3655f436aa009688e7ccb8b7581554fb64278d111f5845e79da8e618/awscli-1.16.14-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 20.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.5MB/s eta 0:00:01
    2% |▊                               | 30kB 1.8MB/s eta 0:00:01
    3% |█                               | 40kB 1.4MB/s eta 0:00:01
---
[01:24:48] 
[01:24:48] failures:
[01:24:48] 
[01:24:48] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11399) stdout ----
[01:24:48] error: unexpected token: `...`
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11402:5
[01:24:48] 5 |     ...
[01:24:48]   |     ^^^
[01:24:48]   |     ^^^
[01:24:48] help: use `..` for an exclusive range
[01:24:48] 5 |     ..
[01:24:48]   |
[01:24:48]   |
[01:24:48] help: or `..=` for an inclusive range
[01:24:48] 5 |     ..=
[01:24:48]   |
[01:24:48] 
[01:24:48] 
[01:24:48] error[E0586]: inclusive range with no end
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11403:1
[01:24:48]   |
[01:24:48] 6 | } // <-- and then freed here
[01:24:48]   |
[01:24:48]   |
[01:24:48]   = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
[01:24:48] 
[01:24:48] error[E0425]: cannot find function `foo` in this scope
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11401:14
[01:24:48]   |
[01:24:48] 4 |     let x = &fo24:48] 3 | let p = bar(&foo()); // <-- temporary is freed after this statement
[01:24:48] 
[01:24:48] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11409)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:24:48] 
[01:24:48] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11417) stdout ----
[01:24:48] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11417) stdout ----
[01:24:48] error: unexpected token: `...`
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11420:1
[01:24:48] 5 | ...
[01:24:48]   | ^^^
[01:24:48]   | ^^^
[01:24:48] help: use `..` for an exclusive range
[01:24:48] 5 | ..
[01:24:48]   |
[01:24:48]   |
[01:24:48] help: or `..=` for an inclusive range
[01:24:48] 5 | ..=
[01:24:48]   |
[01:24:48] 
[01:24:48] 
[01:24:48] error[E0586]: inclusive range with no end
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11421:1
[01:24:48] 6 | }
[01:24:48]   | ^
[01:24:48]   |
[01:24:48]   |
[01:24:48]   = help: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)
[01:24:48] 
[01:24:48] error[E0425]: cannot find function `foo` in this scope
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11418:16
[01:24:48]   |
[01:24:48] 3 | let tmp_ref = &foo(); // freed at end of block
[01:24:48] 
[01:24:48] 
[01:24:48] error[E0425]: cannot find function `bar` in this scope
[01:24:48]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11419:9
[01:24:48]   |
[01:24:48] 4 | let p = bar(tmp_ref);
[01:24:48] 
[01:24:48] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11417)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:24:48] 
[01:24:48] 
---
[01:24:48] 
[01:24:48] 
[01:24:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:48] Build completed unsuccessfully in 0:40:43
[01:24:48] Makefile:58: recipe for target 'check' failed
[01:24:48] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0223b12a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
