plain
[00:03:53]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:04:06]    Compiling libc v0.0.0 (file:///checkout/src/rustc/libc_shim)
[00:04:06]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[00:04:06]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
m: type parameter `<F as core::future::Future>::Output` must be used as the type parameter for some local type (e.g. `MyStruct<<F as core::future::Future>::Output>`)
[00:04:06]     --> liballoc/boxed.rs:999:1
[00:04:06]      |
[00:04:06] 999  | / impl<'a, F> From<Box<F>> for FutureObj<'a, F::Output>
[00:04:06] 1000 | | where
[00:04:06] 1001 | |     F: Future + Send + 'a,
[00:04:06] 1002 | | {
[00:04:06] 1005 | |     }
[00:04:06] 1006 | | }
[00:04:06] 1006 | | }
[00:04:06]      | |_^ type parameter `<F as core::future::Future>::Output` must be used as the type parameter for some local type
[00:04:06]      |
[00:04:06]      = note: only traits defined in the current crate can be implemented for a type parameter
[00:04:06] 
[00:04:06] error[E0210]: type parameter `<F as core::future::Future>::Output` must be used as the type parameter for some local type (e.g. `MyStruct<<F as core::future::Future>::Output>`)
[00:04:06]     --> liballoc/boxed.rs:1009:1
[00:04:06]      |
[00:04:06] 1009 | / impl<'a, F> From<PinBox<F>> for LocalFutureObj<'a, F::Output>
[00:04:06] 1010 | | where
[00:04:06] 1011 | |     F: Future + 'a,
[00:04:06] 1012 | | {
[00:04:06] 1015 | |     }
[00:04:06] 1016 | | }
[00:04:06] 1016 | | }
[00:04:06]      | |_^ type parameter `<F as core::future::Future>::Output` must be used as the type parameter for some local type
[00:04:06]      |
[00:04:06]      = note: only traits defined in the current crate can be implemented for a type parameter
[00:04:06] 
[00:04:06] error[E0210]: type parameter `<F as core::future::Future>::Output` must be used as the type pux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu
241192 ./src/llvm-emscripten
192704 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
177776 ./obj/build/bootstrap/debug/deps
170384 ./obj/build/cache
170384 ./obj/build/cache
170380 ./obj/build/cache/2018-07-27
169712 ./.git
156636 ./.git/modules
156632 ./.git/modules/src
149128 ./src/llvm-emscripten/test
145460 ./obj/build/bootstrap/debug/incremental
130592 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs
130588 ./obj/build/bootstrap/debug/incremental/bootstrap-c7ee2tfsizs/s-f3e3ixv3zk-p0s69n-3t5kexjst7huj
97532 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
77076 ./.git/modules/src/tools
71508 ./src/llvm/lib
70500 ./obj/build/x86_64-unknown-linux-gnu/native
