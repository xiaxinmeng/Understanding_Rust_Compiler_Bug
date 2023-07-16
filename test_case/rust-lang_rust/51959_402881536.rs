plain
    100% |████████████████████████████████| 51kB 8.7MB/s 
Collecting botocore==1.10.51 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e3/ba/f6c9220d87784a85f24a8f2425edccb2f330d15c304ea2373ed8206a03ca/botocore-1.10.51-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 42.7MB/s eta 0:00:01
    0% |▏                               | 20kB 22.9MB/s eta 0:00:01
    0% |▎                               | 30kB 24.8MB/s eta 0:00:01
    0% |▎                               | 40kB 23.2MB/s eta 0:00:01
---
[00:46:10] ....................................................................................................
[00:46:22] ....................................................................................................
[00:46:33] ....................................................................................................
[00:46:55] .....................................................................i..............................
[00:47:08] .............................................F......................................................
[00:47:23] ...i............................F..........................................................i........
[00:47:37] ...........................................................................test [run-pass] run-pass/issue-29227.rs has been running for over 60 seconds
[00:48:06] ....................................................................................................
[00:48:21] ....................................................................................................
[00:48:37] .....................................................................i..............................
[00:48:51] ....................................................................i...............................
---
[00:51:27] ---- [run-pass] run-pass/issue-37109.rs stdout ----
[00:51:27] 
[00:51:27] error: compilation failed!
[00:51:27] status: exit code: 101
[00:51:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-37109.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-37109/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-37109/auxiliary"
[00:51:27] ------------------------------------------
[00:51:27] 
[00:51:27] ------------------------------------------
[00:51:27] stderr:
[00:51:27] stderr:
[00:51:27] ------------------------------------------
[00:51:27] thread 'main' panicked at 'region_obligations not empty: [
[00:51:27]         NodeId(
[00:51:27]             0
[00:51:27]         ),
[00:51:27]         ),
[00:51:27]         RegionObligation(sub_region=ReFree(DefId(0/0:7 ~ issue_37109[317d]::example[0]), BrNamed(crate0:DefIndex(1:12), 'a)), sup_type=&'a T)
[00:51:27]     )
[00:51:27] ]', librustc/infer/mod.rs:1031:9
[00:51:27] 
[00:51:27] error: internal compiler error: unexpected panic
[00:51:27] 
[00:51:27] 
[00:51:27] note: the compiler unexpectedly panicked. this is a bug.
[00:51:27] 
[00:51:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:51:27] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:51:27] 
[00:51:27] 
[00:51:27] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:51:27] 
[00:51:27] ------------------------------------------
[00:51:27] 
[00:51:27] thread '[run-pass] run-pass/issue-37109.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:27] thread '[run-pass] run-pass/issue-37109.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:51:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:27] 
[00:51:27] ---- [run-pass] run-pass/issue-42552.rs stdout ----
[00:51:27] 
[00:51:27] error: compilation failed!
[00:51:27] status: exit code: 101
[00:51:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-42552.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42552/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-42552/auxiliary"
[00:51:27] ------------------------------------------
[00:51:27] 
[00:51:27] ------------------------------------------
[00:51:27] stderr:
[00:51:27] stderr:
[00:51:27] ------------------------------------------
[00:51:27] thread 'main' panicked at 'region_obligations not empty: [
[00:51:27]         NodeId(
[00:51:27]             0
[00:51:27]         ),
[00:51:27]         ),
[00:51:27]         RegionObligation(sub_region='_#10r, sup_type=i32)
[00:51:27]     )
[00:51:27] ]', librustc/infer/mod.rs:1031:9
[00:51:27] 
[00:51:27] error: internal compiler error: unexpected panic
[00:51:27] 
[00:51:27] 
[00:51:27] note: the compiler unexpectedly panicked. this is a bug.
[00:51:27] 
[00:51:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:51:27] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:51:27] 
[00:51:27] 
[00:51:27] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:51:27] 
[00:51:27] ------------------------------------------
[00:51:27] 
[00:51:27] thread '[run-pass] run-pass/issue-42552.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
