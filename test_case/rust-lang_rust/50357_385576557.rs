plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/3f/6c/dbbd5992740649134e597833bea5a95e1fc093a7123e9b8156d838b960e4/awscli-1.15.11-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 7.1MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.8MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.9MB/s 
Collecting botocore==1.10.11 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/15/c0/04ec8aec3cdf7dd4887f2666044550eb3370a4f29668e53519cc7143bdcf/botocore-1.10.11-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 43.4MB/s eta 0:00:01
    0% |▏                               | 20kB 19.1MB/s eta 0:00:01
    0% |▎                               | 30kB 23.9MB/s eta 0:00:01
    0% |▎                               | 40kB 13.0MB/s eta 0:00:01
---
[00:03:43]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:44] warning: unused import: `uninitialized`
[00:03:44]   --> liballoc/arc.rs:25:50
[00:03:44]    |
[00:03:44] 25 | use core::mem::{self, align_of_val, size_of_val, uninitialized};
[00:03:44]    |
[00:03:44]    = note: #[warn(unused_imports)] on by default
[00:03:44] 
[00:03:44]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:44]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:44]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:45] error[E0609]: no field `data` on type `core::option::Option<&arc::ArcInner<T>>`
[00:03:45]    --> liballoc/arc.rs:810:70
[00:03:45]     |
[00:03:45] 810 |                 let mut swap = Arc::new(ptr::read(&weak.ptr.as_ref().data));
[00:03:45] 
[00:03:45] error[E0308]: mismatched types
[00:03:45]     --> liballoc/arc.rs:1055:49
[00:03:45]      |
[00:03:45]      |
[00:03:45] 1055 |                 Ok(_) => return Some(Arc { ptr: self.ptr, phantom: PhantomData }),
[00:03:45]      |                                                 ^^^^^^^^ expected struct `core::ptr::NonNull`, found *-ptr
[00:03:45]      |
[00:03:45]      = note: expected type `core::ptr::NonNull<arc::ArcInner<T>>`
[00:03:45]                 found type `*mut arc::ArcInner<T>`
[00:03:45] 
[00:03:45] error[E0599]: no method named `as_ref` found for type `&arc::ArcInner<T>` in the current scope
[00:03:45]     --> liballoc/arc.rs:1164:78
[00:03:45]      |
[00:03:45] 1164 |                 Global.dealloc(non_null.as_opaque(), Layout::for_value(inner.as_ref()))
[00:03:45]      |
[00:03:45]      |
[00:03:45]      = note: the method `as_ref` exists but the following trait bounds were not satisfied:
[00:03:45]              `&arc::ArcInner<T> : core::convert::AsRef<_>`
[00:03:45]      = help: items from traits can only be used if the trait is implemented and in scope
[00:03:45]      = note: the following trait defines an item `as_ref`, perhaps you need to implement it:
[00:03:45]              candidate #1: `core::convert::AsRef`
[00:03:47] error: aborting due to 3 previous errors
[00:03:47] 
[00:03:47] Some errors occurred: E0308, E0599, E0609.
[00:03:47] For more information about an error, try `rustc --explain E0308`.
[00:03:47] For more information about an error, try `rustc --explain E0308`.
[00:03:47] error: Could not compile `alloc`.
[00:03:47] 
[00:03:47] Caused by:
[00:03:47]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc liballoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1e8fdab57e52df69 -C extra-filename=-1e8fdab57e52df69 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-05c2804dd4e2fee5.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-4409ca235794bde7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-6e02f9e6ea3b48c2/out` (exit code: 101)
[00:03:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:47] expected success, got: exit code: 101
[00:03:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:47] travis_fold:end:stage0-std

[00:03:47] travis_time:end:stage0-std:start=1525137949890174379,finish=1525137987852178010,duration=37962003631


[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] Build completed unsuccessfully in 0:00:39
[00:03:47] make: *** [tidy] Error 1
[00:03:47] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00fca092
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
