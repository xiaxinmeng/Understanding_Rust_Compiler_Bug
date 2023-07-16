plain
.................................................................................................... 9700/12259
....................F............................................................................... 9800/12259
.............................................................................................ii.i... 9900/12259
.................................................................................................... 10000/12259
.............F..........................................................................iiiiii.i..ii 10100/12259
.................................................................................................... 10300/12259
.................................................................................................... 10400/12259
.................................................................................................... 10500/12259
.................................................................................................... 10600/12259
---
failures:

---- [ui] ui/blind-item-mixed-crate-use-item.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/blind-item-mixed-crate-use-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/blind-item-mixed-crate-use-item/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/blind-item-mixed-crate-use-item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot satisfy dependencies so `std` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `libc` only shows up once
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `cfg_if` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `hashbrown` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_demangle` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `std_detect` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `addr2line` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `gimli` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `object` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `memchr` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `miniz_oxide` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `adler` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `panic_unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away
error: aborting due to 19 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/inference/issue-81522.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-81522.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-81522/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-81522/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot satisfy dependencies so `std` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `libc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `cfg_if` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `hashbrown` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_demangle` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `std_detect` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `addr2line` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `gimli` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `object` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `memchr` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `miniz_oxide` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `adler` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `panic_unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away
error: aborting due to 19 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/inference/inference_unstable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference_unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:16:20
   |
LL |     assert_eq!('x'.ipu_flatten(), 1);
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_flatten(...)` to keep using the current method
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_flatten`
warning: an associated constant with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:19:16
   |
   |
LL |     assert_eq!(char::C, 1);
   |                ^^^^^^^ help: use the fully qualified path to the associated const: `<char as IpuItertools>::C`
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: add `#![feature(assoc_const_ipu_iter)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::C`

error: cannot satisfy dependencies so `std` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `libc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `cfg_if` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `hashbrown` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_demangle` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `std_detect` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `addr2line` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `gimli` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `object` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `memchr` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `miniz_oxide` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `adler` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `panic_unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away
error: aborting due to 19 previous errors; 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-12612.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12612.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12612/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12612/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot satisfy dependencies so `std` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `libc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `cfg_if` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `hashbrown` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_demangle` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `std_detect` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `addr2line` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `gimli` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `object` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `memchr` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `miniz_oxide` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `adler` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `panic_unwind` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away
error: aborting due to 19 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-70093.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-70093.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70093/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zlink-native-libraries=no" "-Cdefault-linker-libraries=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70093/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70093/a.issue_70093.82f50709-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70093/a.17forna0i7d1zh36.rcgu.o" "-Wl,--as-needed" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-7a374b96d06c424c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-4f1edb529abb76e8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-f182e040509242cd.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-8777e23b5da50cbc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-c3411f56440a8920.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-a7e9cc394034aa4f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-3f4ba2c5ec5fe2d9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-cca71b464f3f6e82.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-aa6c3c5dec14bf3c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-287e8d2a07a05f7c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-5f1f9f0ab2c97a8f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3fa75beb688f798.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-c100cb6aa09ec671.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-9bfe9d94484c546a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-5f56516096d51b90.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0ae23430f49edc9e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0286f51ede2a0cbe.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-cfcd5a8ed441a0ab.rlib" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70093/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `core::ptr::drop_in_place<std::sys_common::rwlock::StaticRWLockReadGuard>':
           std.bc1d14b1-cgu.0:(.text._ZN4core3ptr67drop_in_place$LT$std..sys_common..rwlock..StaticRWLockReadGuard$GT$17hd22c9bc738e0b102E+0x15): undefined reference to `pthread_rwlock_unlock'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sync::once::Once::call_once::{{closure}}':
           std.bc1d14b1-cgu.0:(.text._ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17hbde0cebbb714a865E+0x4a): undefined reference to `pthread_mutex_trylock'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::rt::init':
           std.bc1d14b1-cgu.0:(.text._ZN3std2rt4init17h77fefdd6a4d14bacE+0x26a): undefined reference to `pthread_getattr_np'
           std.bc1d14b1-cgu.0:(.text._ZN3std2rt4init17h77fefdd6a4d14bacE+0x299): undefined reference to `pthread_attr_getstack'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sys_common::thread_local_dtor::register_dtor_fallback::run_dtors':
           std.bc1d14b1-cgu.0:(.text._ZN3std10sys_common17thread_local_dtor22register_dtor_fallback9run_dtors17hee00a5d971c5ecfeE+0x9f): undefined reference to `pthread_getspecific'
           std.bc1d14b1-cgu.0:(.text._ZN3std10sys_common17thread_local_dtor22register_dtor_fallback9run_dtors17hee00a5d971c5ecfeE+0xc5): undefined reference to `pthread_setspecific'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sys_common::thread_local_key::StaticKey::lazy_init':
           std.bc1d14b1-cgu.0:(.text._ZN3std10sys_common16thread_local_key9StaticKey9lazy_init17hc886aba6d20f6c26E+0x21): undefined reference to `pthread_key_create'
           std.bc1d14b1-cgu.0:(.text._ZN3std10sys_common16thread_local_key9StaticKey9lazy_init17hc886aba6d20f6c26E+0x4b): undefined reference to `pthread_key_create'
           std.bc1d14b1-cgu.0:(.text._ZN3std10sys_common16thread_local_key9StaticKey9lazy_init17hc886aba6d20f6c26E+0x5f): undefined reference to `pthread_key_delete'
           std.bc1d14b1-cgu.0:(.text._ZN3std10sys_common16thread_local_key9StaticKey9lazy_init17hc886aba6d20f6c26E+0x7f): undefined reference to `pthread_key_delete'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::panicking::rust_panic_with_hook':
           std.bc1d14b1-cgu.0:(.text._ZN3std9panicking20rust_panic_with_hook17hb55dcd05da7622edE+0x195): undefined reference to `pthread_rwlock_unlock'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sys::unix::weak::Weak<F>::initialize':
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix4weak13Weak$LT$F$GT$10initialize17h491d6e1b4f9e3225E+0x3c): undefined reference to `dlsym'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sys::unix::os::getenv':
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix2os6getenv17haf09740a61a06f19E+0x1a1): undefined reference to `pthread_rwlock_unlock'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sys::unix::rwlock::RWLock::read':
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix6rwlock6RWLock4read17hb4547eda265953f9E+0xa): undefined reference to `pthread_rwlock_rdlock'
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix6rwlock6RWLock4read17hb4547eda265953f9E+0x6f): undefined reference to `pthread_rwlock_unlock'
           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdf8894991793710.rlib(std-bdf8894991793710.std.bc1d14b1-cgu.0.rcgu.o): In function `std::sys::unix::thread_local_dtor::register_dtor':
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix17thread_local_dtor13register_dtor17h9016848c3bf82245E+0x27): undefined reference to `pthread_getspecific'
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix17thread_local_dtor13register_dtor17h9016848c3bf82245E+0x64): undefined reference to `pthread_getspecific'
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix17thread_local_dtor13register_dtor17h9016848c3bf82245E+0xb8): undefined reference to `pthread_setspecific'
           std.bc1d14b1-cgu.0:(.text._ZN3std3sys4unix17thread_local_dtor13register_dtor17h9016848c3bf82245E+0xe2): undefined reference to `pthread_getspecific'
           /usr/bin/ld generated: undefined reference to `pthread_getspecific'
           /usr/bin/ld generated: undefined reference to `pthread_key_create'
           /usr/bin/ld generated: undefined reference to `pthread_setspecific'
           collect2: error: ld returned 1 exit status
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/panic-runtime/abort-link-to-unwind-dylib.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort-link-to-unwind-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwind-dylib" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-C" "prefer-dynamic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwind-dylib/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/resolve/extern-prelude.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/extern-prelude.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/extern-prelude" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "extern_prelude" "--extern" "Vec" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/extern-prelude/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot satisfy dependencies so `std` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `compiler_builtins` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `rustc_std_workspace_core` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away

error: cannot satisfy dependencies so `alloc` only shows up once
   |
   = help: having upstream crates all available in one format will likely make this go away
---
test result: FAILED. 12135 passed; 9 failed; 115 ignored; 0 measured; 0 filtered out; finished in 158.45s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:16
