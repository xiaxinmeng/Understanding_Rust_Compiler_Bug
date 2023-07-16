plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/54/a0/dd89b5ae729ac8aeeb446622604e49c2bd97b7fef3d48f2da2d3bb524e55/awscli-1.15.7-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/76/98/b3772fa3aa70d441acfcaf41385b8dc1d0fe4a72e42ac8b020e6aab6e891/botocore-1.10.7-py2.py3-none-any.whl (4.2MB)
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
[00:03:55] [RUSTC-TIMING] alloc_jemalloc test:false 0.191
[00:04:02] [RUSTC-TIMING] alloc test:false 6.816
[00:04:02]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:04:02] [RUSTC-TIMING] panic_unwind test:false 0.271
[00:04:07] error[E0277]: the trait bound `&u16: core::cmp::PartialEq<u16>` is not satisfied
[00:04:07]    --> libstd/sys/windows/os.rs:200:44
[00:04:07]     |
[00:04:07] 200 |         let semicolons = data.filter(|b| b == (';' as u16)).count();
[00:04:07]     |                                            ^^ can't compare `&u16` with `u16`
[00:04:07]     |
[00:04:07]     = help: the trait `core::cmp::PartialEq<u16>` is not implemented for `&u16`
[00:04:07] error: aborting due to previous error
[00:04:07] 
[00:04:07] For more information about this error, try `rustc --explain E0277`.
[00:04:07] [RUSTC-TIMING] std test:false 4.908
[00:04:07] [RUSTC-TIMING] std test:false 4.908
[00:04:07] error: Could not compile `std`.
[00:04:07] 
[00:04:07] Caused by:
[00:04:07]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=468968c2dfd1f66f -C extra-filename=-468968c2dfd1f66f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcore-8b3bce652622b235.rmeta --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libstd_unicode-830d2308368f417f.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libpanic_unwind-f3e7c3dc7c2708ab.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libunwind-58ef4a4c55193d82.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc_jemalloc-d183733f4d204e62.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc-f6e9049eef93479d.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libpanic_abort-74a515719294f544.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcompiler_builtins-2c84d9450866a271.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc_system-372ef3f14ce624b1.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liblibc-d8f546a38c967152.rmeta -L native=/checkout/obj/build/i686-pc-windows-gnu/native/libbacktrace/.libs -l static=backtrace -l advapi32 -l ws2_32 -l userenv -l shell32 -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/build/compiler_builtins-62bfd7e355d8aacf/out -L native=/checkout/obj/build/i686-pc-windows-gnu/native/jemalloc/lib` (exit code: 101)

[00:04:07] travis_time:end:stage0-std:start=1524524164647791195,finish=1524524209481970929,duration=44834179734


[00:04:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:07] expected success, got: exit code: 101
[00:04:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
[00:04:07] Build completed unsuccessfully in 0:00:46
travis_time:end:1290e298:start=1524523962213070174,finish=1524524209741363438,duration=247528293264

