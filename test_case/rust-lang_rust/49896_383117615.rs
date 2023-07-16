plain
[00:01:05] configure: llvm.assertions      := True
[00:01:05] configure: build.locked-deps    := True
[00:01:05] configure: llvm.ccache          := sccache
[00:01:05] configure: build.openssl-static := True
[00:01:05] configure: build.configure-args := ['--enable-quiet-tests', '--enable-sccache', ' ...
[00:01:05] configure: writing `config.toml` in current directory
[00:01:05] configure: 
[00:01:05] configure: run `python /checkout/x.py --help`
[00:01:05] configure: 
---
[00:03:42]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:43]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:43]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:48]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:56] error[E0599]: no method named `make_ascii_uppercase` found for type `sys_common::wtf8::Wtf8Buf` in the current scope
[00:03:56]    --> libstd/sys/windows/process.rs:47:13
[00:03:56]     |
[00:03:56] 47  |         buf.make_ascii_uppercase();
[00:03:56]     | 
[00:03:56]    ::: libstd/sys_common/wtf8.rs:124:1
[00:03:56]     |
[00:03:56]     |
[00:03:56] 124 | pub struct Wtf8Buf {
[00:03:56]     | ------------------ method `make_ascii_uppercase` not found for this
[00:03:56]     |
[00:03:56]     = help: items from traits can only be used if the trait is implemented and in scope
[00:03:56]     = note: the following trait defines an item `make_ascii_uppercase`, perhaps you need to implement it:
[00:03:56]             candidate #1: `ascii::AsciiExt`
[00:03:56] error: aborting due to previous error
[00:03:56] 
[00:03:56] For more information about this error, try `rustc --explain E0599`.
[00:03:56] error: Could not compile `std`.
[00:03:56] error: Could not compile `std`.
[00:03:56] 
[00:03:56] Caused by:
[00:03:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=6197ecaef2e5f859 -C extra-filename=-6197ecaef2e5f859 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc-2672b81fffcd4465.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libpanic_abort-967ca3b04cf67812.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcore-8b3bce652622b235.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liblibc-1d790924b0f850a1.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libpanic_unwind-6f3c21a7e6e077db.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libunwind-beabc96c806236c2.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc_system-3ec3b5e4e22a73d4.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libcompiler_builtins-7e916d513ccde52f.rmeta --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/libstd_unicode-b49cd7311cec7b59.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/deps/liballoc_jemalloc-8209c87b42049987.rmeta -L native=/checkout/obj/build/i686-pc-windows-gnu/native/libbacktrace/.libs -l static=backtrace -l advapi32 -l ws2_32 -l userenv -l shell32 -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/build/compiler_builtins-6789d0ad544f7553/out -L native=/checkout/obj/build/i686-pc-windows-gnu/native/jemalloc/lib` (exit code: 101)
[00:03:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:56] expected success, got: exit code: 101
[00:03:56] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:56] travis_fold:end:stage0-std

[00:03:56] travis_time:end:stage0-std:start=1524230006625390912,finish=1524230048685810616,duration=42060419704

