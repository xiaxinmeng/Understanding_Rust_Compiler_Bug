plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/5f/86/363f1249d0b62d7cce3cb8973fb6715b57ca75f8425d6c45fe5e129531a9/awscli-1.15.33-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
[00:52:05] [RUSTC-TIMING] alloc_jemalloc test:false 0.092
[00:52:09] [RUSTC-TIMING] alloc test:false 5.479
[00:52:09]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:52:09] [RUSTC-TIMING] panic_unwind test:false 0.264
[00:52:10] error[E0425]: cannot find value `FDIO_SPAWN_SHARE_JOB` in this scope
[00:52:10]   --> libstd/sys/unix/process/process_fuchsia.rs:97:13
[00:52:10]    |
[00:52:10] 97 |             FDIO_SPAWN_SHARE_JOB | FDIO_SPAWN_CLONE_LDSVC | FDIO_SPAWN_CLONE_NAMESPACE,
[00:52:10]    |             ^^^^^^^^^^^^^^^^^^^^ did you mean `FDIO_SPAWN_CLONE_JOB`?
[00:52:13] error: aborting due to previous error
[00:52:13] 
[00:52:13] For more information about this error, try `rustc --explain E0425`.
[00:52:13] [RUSTC-TIMING] std test:false 4.443
[00:52:13] [RUSTC-TIMING] std test:false 4.443
[00:52:13] error: Could not compile `std`.
[00:52:13] 
[00:52:13] Caused by:
[00:52:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=b5b74e5bdff83463 -C extra-filename=-b5b74e5bdff83463 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps --target x86_64-unknown-fuchsia -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_unwind-3c933c054569597f.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libunwind-eb99d5719502c8b0.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcore-0afe35f86d76f813.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc-d2a4d023e4145346.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liblibc-d1ac84fc4164a85d.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libstd_unicode-5d1869050fab4dc6.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libcompiler_builtins-d0c6c868059af195.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc_system-0361c41b87a7960e.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/liballoc_jemalloc-bba24cf25833c139.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/deps/libpanic_abort-e6f744d63d1646cb.rlib -l backtrace -l zircon -l fdio -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-fuchsia/release/build/compiler_builtins-bf3e53f96a081c7f/out` (exit code: 101)
[00:52:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:52:13] expected success, got: exit code: 101
[00:52:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:52:13] travis_fold:end:stage2-std

[00:52:13] travis_time:end:stage2-std:start=1528262849172730500,finish=1528262911793853395,duration=62621122895

