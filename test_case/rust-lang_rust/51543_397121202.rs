plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e2/29/f5b58658602baf037db0f650567d95e8d36104e1bd1966fa4628d7c7e470/awscli-1.15.38-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:45] 
[00:47:45] running 3024 tests
[00:47:59] ......F.............................................................................................
[00:48:30] ....................................................................................................
[00:48:45] ....................................................................................................
[00:48:57] ....................................................................................................
[00:49:18] ....................................................................................................
---
[00:53:13] .......................................................i............................................
[00:53:33] ......................................................i.............................................
[00:54:02] ..................................................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:02] ..
[00:54:17] ........F........................................................F..................................
[00:55:17] ...........................................i.ii.....................................................
[00:55:47] ...................................................iiiiiii..........................................
[00:56:06] ....................................................................................................
[00:56:23] ....................................................................................................
---
[00:56:52] ---- [run-pass] run-pass/realloc-16687.rs stdout ----
[00:56:52] 
[00:56:52] error: compilation failed!
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/realloc-16687.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/realloc-16687/auxiliary"
[00:56:52] ------------------------------------------
[00:56:52] 
[00:56:52] ------------------------------------------
[00:56:52] stderr:
[00:56:52] stderr:
[00:56:52] ------------------------------------------
[00:56:52] error[E0432]: unresolved import `std::alloc::oom`
[00:56:52]   --> /checkout/src/test/run-pass/realloc-16687.rs:18:41
[00:56:52]    |
[00:56:52] 18 | use std::alloc::{Global, Alloc, Layout, oom};
[00:56:52]    |                                         ^^^ no `oom` in `alloc`
[00:56:52] error: aborting due to previous error
[00:56:52] 
[00:56:52] For more information about this error, try `rustc --explain E0432`.
[00:56:52] 
---
[00:56:52] ---- [run-pass] run-pass/regions-mock-codegen.rs stdout ----
[00:56:52] 
[00:56:52] error: compilation failed!
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/regions-mock-codegen.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-mock-codegen/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/regions-mock-codegen/auxiliary"
[00:56:52] ------------------------------------------
[00:56:52] 
[00:56:52] ------------------------------------------
[00:56:52] stderr:
[00:56:52] stderr:
[00:56:52] ------------------------------------------
[00:56:52] error[E0432]: unresolved import `std::alloc::oom`
[00:56:52]   --> /checkout/src/test/run-pass/regions-mock-codegen.rs:15:41
[00:56:52]    |
[00:56:52] 15 | use std::alloc::{Alloc, Global, Layout, oom};
[00:56:52]    |                                         ^^^ no `oom` in `alloc`
[00:56:52] error: aborting due to previous error
[00:56:52] 
[00:56:52] For more information about this error, try `rustc --explain E0432`.
[00:56:52] 
[00:56:52] 
[00:56:52] ------------------------------------------
[00:56:52] 
[00:56:52] thread '[run-pass] run-pass/regions-mock-codegen.rs' panickWed, 13 Jun 2018 23:37:35 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
