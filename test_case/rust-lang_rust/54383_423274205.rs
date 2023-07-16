plain
[00:40:48]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:41:01] warning: `[T]` cannot be resolved, ignoring it...
[00:41:01]   --> libcore/ops/unsize.rs:89:9
[00:41:01]    |
[00:41:01] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:41:01]    |
[00:41:01]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:41:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:41:01] 
[00:41:01] 
[00:41:01] warning: `[T]` cannot be resolved, ignoring it...
[00:41:01]   --> libcore/ops/unsize.rs:89:9
[00:41:01]    |
[00:41:01] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:41:01]    |
[00:41:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:41:01] 
[00:41:19]     Checking compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
---
[00:41:21]     Checking alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:41:23] warning: `[T]` cannot be resolved, ignoring it...
[00:41:23]   --> /checkout/src/libcore/ops/unsize.rs:89:9
[00:41:23]    |
[00:41:23] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:41:23]    |
[00:41:23]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:41:23]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:41:23] 
---
[00:41:29] 
[00:41:29] warning: `[T]` cannot be resolved, ignoring it...
[00:41:29]   --> /checkout/src/libcore/ops/unsize.rs:89:9
[00:41:29]    |
[00:41:29] 89 | /// - &[T] is CoerceSized<&[T; N]> for any N
[00:41:29]    |
[00:41:29]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:41:29] 
[00:41:29] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
---
[00:49:23] ....................................................................................................
[00:49:26] ......................................................i.............................................
[00:49:28] ....................................................................................................
[00:49:31] ....................................................................................................
[00:49:33] ..iiiiiiiii.........................................................................................
[00:49:39] ....................................................................................................
[00:49:41] .....................................................................................i..............
[00:49:44] ....................................................................................................
[00:49:46] ........................................i.i..ii.....................................................
---
[01:16:30]   |
[01:16:30] 3 | use std::marker::PhantomData;
[01:16:30]   |
[01:16:30] 
[01:16:30] error[E0378]: the trait `CoerceSized` may only be implemented for structs containing the field being coerced, `PhantomData` fields, and nothing else
[01:16:30]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5599:1
[01:16:30]    |
[01:16:30] 16 | / impl<T, U> CoerceSized<Wrapper<T>> for Wrapper<U>
[01:16:30] 17 | | where
[01:16:30] 18 | |     T: CoerceUnsized<U>,
[01:16:30] 19 | |     U: CoerceSized<T>,
[01:16:30] 20 | | {}
[01:16:30]    |
[01:16:30]    |
[01:16:30]    = note: extra field `_phantom` of type `[type error]` is not allowed
[01:16:30] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0378 (line 5585)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:16:30] 
[01:16:30] 
[01:16:30] failures:
---
[01:16:30] 
[01:16:30] 
[01:16:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:30] Build completed unsuccessfully in 0:34:23
[01:16:30] make: *** [check] Erro5381060 .
2984884 ./obj/build
2365004 ./obj/build/x86_64-unknown-linux-gnu
1192992 ./.git
1067680 ./src
---
162932 ./.git/modules/src/tools/lldb/objects/pack
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148628 ./obj/build/bootstrap/debug/incremental
142256 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86 printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0655edaa
travis_time:start:0655edaa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0729ee22
$ dmesg | grep -i kill
