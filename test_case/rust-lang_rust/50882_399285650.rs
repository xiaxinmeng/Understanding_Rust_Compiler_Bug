plain
    96% |███████████████████████████████ | 204kB 30.1MB/s eta 0:00:01
    100% |████████████████████████████████| 215kB 4.9MB/s 
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1->botocore==1.10.43->awscli)
Building wheels for collected packages: botocore
  Running setup.py bdist_wheel for botocore ... - \ | / - \ | / - \ done
Successfully built botocore
Installing collected packages: colorama, pyasn1, rsa, futures, jmespath, docutils, python-dateutil, botocore, s3transfer, awscli
Successfully installed awscli-1.15.43 botocore-1.10.43 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:03:49]    |                                    ^^^ no `oom` in `alloc`
[00:03:49] 
[00:03:49]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:49]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:50] error[E0599]: no method named `as_opaque` found for type `core::ptr::NonNull<T>` in the current scope
[00:03:50]     |
[00:03:50]     |
[00:03:50] 180 |         a.dealloc(NonNull::new_unchecked(ptr).as_opaque(), layout);
[00:03:50] 
[00:03:50] error[E0308]: mismatched types
[00:03:50]    --> liballoc/boxed.rs:922:25
[00:03:50]     |
[00:03:50]     |
[00:03:50] 922 |         PinBox { inner: Box::from_raw(raw) }
[00:03:50]     |                         ^^^^^^^^^^^^^^^^^^ expected type parameter, found struct `alloc::Global`
[00:03:50]     |
[00:03:50]     = note: expected type `boxed::Box<_, A>`
[00:03:50]                found type `boxed::Box<_, alloc::Global>`
[00:03:51] error: aborting due to 3 previous errors
[00:03:51] 
[00:03:51] Some errors occurred: E0308, E0432, E0599.
[00:03:51] For more information about an error, try `rustc --explain E0308`.
[00:03:51] For more information about an error, try `rustc --explain E0308`.
[00:03:51] error: Could not compile `alloc`.
[00:03:51] 
[00:03:51] Caused by:
[00:03:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=d6cd5f8b78fddf12 -C extra-filename=-d6cd5f8b78fddf12 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-90a13cda2e54742f.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-e9cdce497aae9e81.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-ffd422941bf53e42/out` (exit code: 101)
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:51] expected success, got: exit code: 101
[00:03:51] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:51] travis_fold:end:stage0-std

[00:03:51] travis_time:end:stage0-std:start=1529627971064917025,finish=1529628002012187342,duration=30947270317


[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:00:32
[00:03:51] make: *** [tidy] Error 1
[00:03:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0049bdd4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:123eab98:start=1529628002652879656,finish=1529628002661221531,duration=8341875
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a0bc3c9
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c393510
$ dmesg | grep -i kill
