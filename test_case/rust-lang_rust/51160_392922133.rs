plain
[00:09:12] configure: llvm.assertions      := True
[00:09:12] configure: build.locked-deps    := True
[00:09:12] configure: llvm.ccache          := sccache
[00:09:12] configure: build.openssl-static := True
[00:09:12] configure: build.configure-args := ['--enable-quiet-tests', '--enable-sccache', ' ...
[00:09:12] configure: writing `config.toml` in current directory
[00:09:12] configure: 
[00:09:12] configure: run `python /checkout/x.py --help`
[00:09:12] configure: 
---
[00:15:07]     Checking libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:15:07]     Checking alloc v0.0.0 (file:///checkout/src/liballoc)
[00:15:07]     Checking std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:15:08]     Checking alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:15:08] error[E0605]: non-primitive cast: `core::ptr::NonNull<core::alloc::Opaque>` as `*mut u8`
[00:15:08]    --> liballoc_system/lib.rs:287:54
[00:15:08]     |
[00:15:08] 287 |                     HeapReAlloc(GetProcessHeap(), 0, ptr as LPVOID, new_size) as *mut Opaque
[00:15:08]     |
[00:15:08]     = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[00:15:08] 
[00:15:08] error: aborting due to previous error
[00:15:08] error: aborting due to previous error
[00:15:08] 
[00:15:08] For more information about this error, try `rustc --explain E0605`.
[00:15:08] error: Could not compile `alloc_system`.
[00:15:08] 
[00:15:08] Caused by:
[00:15:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc_system liballoc_system/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C metadata=920c1d87f1854d18 -C extra-filename=-920c1d87f1854d18 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcompiler_builtins-e74d4e2453f9ac82.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liblibc-2924b7d63d95589e.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcore-0e63b5d8cba5e64e.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/build/compiler_builtins-7a2aed77e6522e53/out` (exit code: 101)
[00:16:15] error: build failed
[00:16:15] error: build failed
[00:16:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:16:15] expected success, got: exit code: 101
[00:16:15] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:16:15] travis_fold:end:stage0-std

[00:16:15] travis_time:end:stage0-std:start=1527619406781149225,finish=1527619498288126868,duration=91506977643

