plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/40/7b/dd2e1823627f38afb7e92e4e9792d81902c084ab7501c1f748169a5336ae/awscli-1.15.29-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 22.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:53:45] .................................................................i..................................
[00:53:49] ....................................................................................................
[00:53:55] ....................................................................................................
[00:54:02] ..............................................................................................i.....
[00:54:04] ............iiiiiiiii...................................................
[00:54:04] 
[00:54:04] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:54:53] .................................................................i..................................
[00:54:57] ....................................................................................................
[00:55:02] ....................................................................................................
[00:55:08] ..............................................................................................i.....
[00:55:11] ............iiiiiiiii...................................................
[00:55:11] 
[00:55:11]  finished in 66.297
[00:55:11] travis_fold:end:test_ui_nll

---
[01:35:30] test /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) ... FAILED
[01:35:30] 
[01:35:30] failures:
[01:35:30] 
[01:35:30] ---- /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13) stdout ----
[01:35:30] error[E0412]: cannot find type `Any` in this scope
[01:35:30]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:17:16
[01:35:30]   |
[01:35:30] 5 |     let x: Box<Any> = Box::new(42);
[01:35:30]   |                ^^^ not found in this scope
[01:35:30] help: possible candidate is found in another module, you can import it into scope
[01:35:30] 4 | use std::any::Any;
[01:35:30]   |
[01:35:30] 
[01:35:30] 
[01:35:30] error[E0412]: cannot find type `Any` in this scope
[01:35:30]  --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:18:12
[01:35:30]   |
[01:35:30] 6 |     let x: Any = *x;
[01:35:30]   |            ^^^ not found in this scope
[01:35:30] help: possible candidate is found in another module, you can import it into scope
[01:35:30] 4 | use std::any::Any;
[01:35:30]   |
[01:35:30] 
[01:35:30] 
[01:35:30] error[E0412]: cannot find type `Any` in this scope
[01:35:30]   --> /checkout/src/doc/unstable-book/src/language-features/unsized-locals.md:24:11
[01:35:30]    |
[01:35:30] 12 | fn foo(_: Any) {}
[01:35:30]    |           ^^^ not found in this scope
[01:35:30] help: possible candidate is found in another module, you can import it into scope
[01:35:30] 4  | use std::any::Any;
[01:35:30]    |
[01:35:30] 
[01:35:30] thread '/checkout/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals (line 13)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
---
[01:35:30] 
[01:35:30] 
[01:35:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:30] Build completed unsuccessfully in 0:44:01
[01:35:30] make: *** [check] Error 1
[01:35:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:042f0a40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
