plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b5/dd/84d32d2275ea16daf09d561858dd0e615c56c9e8afb2e9b42d02bc45e417/awscli-1.15.51-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 16.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 1.9MB/s 
Collecting botocore==1.10.50 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d5/9f/2e701a365b5ff0e8b664d6c393f3c61c20e52bb5148bbc2e27d737b890db/botocore-1.10.50-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 35.7MB/s eta 0:00:01
    0% |▏                               | 20kB 34.1MB/s eta 0:00:01
    0% |▎                               | 30kB 40.8MB/s eta 0:00:01
    0% |▎                               | 40kB 44.5MB/s eta 0:00:01
---
[00:21:04]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:21:05]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:21:06]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:21:10]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]     |
[00:21:16]     |
[00:21:16] 701 |     print_to(args, &LOCAL_STDOUT, stdout, "stdout");
[00:21:16]     |                     ^^^^^^^^^^^^                   - temporary value only lives until here
[00:21:16]     |                     |
[00:21:16]     |                     temporary value does not live long enough
[00:21:16]     |
[00:21:16]     = note: borrowed value must be valid for the static lifetime...
[00:21:16]     = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]     |
[00:21:16]     |
[00:21:16] 710 |     print_to(args, &LOCAL_STDERR, stderr, "stderr");
[00:21:16]     |                     ^^^^^^^^^^^^                   - temporary value only lives until here
[00:21:16]     |                     |
[00:21:16]     |                     temporary value does not live long enough
[00:21:16]     |
[00:21:16]     = note: borrowed value must be valid for the static lifetime...
[00:21:16]     = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]   --> libstd/future.rs:61:9
[00:21:16]    |
[00:21:16] 61 |         TLS_CX.with(|tls_cx| {
[00:21:16]    |         ^^^^^^ temporary value does not live long enough
[00:21:16] 62 |             tls_cx.set(self.0.take());
[00:21:16] 63 |         });
[00:21:16]    |           - temporary value only lives until here
[00:21:16]    |
[00:21:16]    = note: borrowed value must be valid for the static lifetime...
[00:21:16]    = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]   --> libstd/future.rs:73:18
[00:21:16]    |
[00:21:16] 73 |     let old_cx = TLS_CX.with(|tls_cx| {
[00:21:16]    |                  ^^^^^^ temporary value does not live long enough
[00:21:16] 80 |     });
[00:21:16] 80 |     });
[00:21:16]    |       - temporary value only lives until here
[00:21:16]    |
[00:21:16]    = note: borrowed value must be valid for the static lifetime...
[00:21:16]    = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]    --> libstd/future.rs:96:18
[00:21:16]     |
[00:21:16] 96  |     let cx_ptr = TLS_CX.with(|tls_cx| {
[00:21:16]     |                  ^^^^^^ temporary value does not live long enough
[00:21:16] 100 |     });
[00:21:16] 100 |     });
[00:21:16]     |       - temporary value only lives until here
[00:21:16]     |
[00:21:16]     = note: borrowed value must be valid for the static lifetime...
[00:21:16]     = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]    |
[00:21:16]    |
[00:21:16] 47 |     THREAD_INFO.with(|c| assert!(c.borrow().is_none()));
[00:21:16]    |     ^^^^^^^^^^^                                        - temporary value only lives until here
[00:21:16]    |     |
[00:21:16]    |     temporary value does not live long enough
[00:21:16]    |
[00:21:16]    = note: borrowed value must be valid for the static lifetime...
[00:21:16]    = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]    |
[00:21:16]    |
[00:21:16] 48 |     THREAD_INFO.with(move |c| *c.borrow_mut() = Some(ThreadInfo{
[00:21:16]    |     ^^^^^^^^^^^ temporary value does not live long enough
[00:21:16] 51 |     }));
[00:21:16] 51 |     }));
[00:21:16]    |        - temporary value only lives until here
[00:21:16]    |
[00:21:16]    = note: borrowed value must be valid for the static lifetime...
[00:21:16]    = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]    |
[00:21:16]    |
[00:21:16] 55 |     THREAD_INFO.with(move |c| c.borrow_mut().as_mut().unwrap().stack_guard = stack_guard);
[00:21:16]    |     ^^^^^^^^^^^ temporary value does not live long enough                                - temporary value only lives until here
[00:21:16]    |
[00:21:16]    = note: borrowed value must be valid for the static lifetime...
[00:21:16]    = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]     |
[00:21:16]     |
[00:21:16] 218 |     let prev = LOCAL_STDERR.with(|s| s.borrow_mut().take());
[00:21:16]     |                ^^^^^^^^^^^^                                - temporary value only lives until here
[00:21:16]     |                |
[00:21:16]     |                temporary value does not live long enough
[00:21:16]     |
[00:21:16]     = note: borrowed value must be valid for the static lifetime...
[00:21:16]     = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]     |
[00:21:16]     |
[00:21:16] 223 |            LOCAL_STDERR.with(|slot| {
[00:21:16]     |            ^^^^^^^^^^^^ temporary value does not live long enough
[00:21:16] 224 |                *slot.borrow_mut() = s.take();
[00:21:16] 225 |            });
[00:21:16]     |              - temporary value only lives until here
[00:21:16]     |
[00:21:16]     = note: borrowed value must be valid for the static lifetime...
[00:21:16]     = note: consider using a `let` binding to increase its lifetime
[00:21:16] 
[00:21:16] error[E0597]: borrowed value does not live long enough
[00:21:16]     m| |             }
[00:21:16] 435 | |         }
[00:21:16]     | |_________^
[00:21:16] 
[00:21:16] 
[00:21:17] error: aborting due to 11 previous errors
[00:21:17] 
[00:21:17] For more information about this error, try `rustc --explain E0597`.
[00:21:17] error: Could not compile `std`.
[00:21:17] 
[00:21:17] Caused by:
[00:21:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=84c7fbfce53d7ef5 -C extra-filename=-84c7fbfce53d7ef5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c69d0f052402a17c.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-07d26af630202296.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-4df9e82ac7dca73d.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-30cc8e27b6fe75eb.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-9d37d95f0c4f2954.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-f8b3dd8e60562e7e.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-d58b9a962a6b502c.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-44d3e8b90f7d78f1.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-457ec772a506b6ac.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0226c7120ae54e2a.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-faa081c0f92f0e3b.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-f430c0b2bfd2c0fa.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-e5529cb27937bb7c.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-61a63a2307487562.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/ -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-0a2e62b135669011/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:21:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:17] expected success, got: exit code: 101
[00:21:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1114:9
[00:21:17] travis_fold:end:stage1-std

[00:21:17] travis_time:end:stage1-std:start=1530655862089009495,finish=1530655935962610544,duration=73873601049


[00:21:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:17] Build completed unsuccessfully in 0:16:19
[00:21:17] make: *** [all] Error 1
[00:21:17] Makefile:28: recipe for target 'all' failed
342268 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu
315048 ./src/llvm
241184 ./src/llvm-emscripten
210228 ./src/llvm/test
---
158324 ./.git/modules/src
149120 ./src/llvm-emscripten/test
144328 ./obj/build/bootstrap/debug/incremental
129816 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9
129812 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9/s-f2kl3vv06w-dk09c0-2cm8xlrxnbnm8
97524 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
89820 ./src/llvm/test/CodeGen
88912 ./obj/build/x86_64-unknown-linux-gnu/stage1
88888 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
