
Thread 2 (Thread 0x7fa8a6fff700 (LWP 14924)):
#0  0x00007fa89ce685ea in <proc_macro::TokenStream as core::iter::traits::Extend<proc_macro::TokenStream>>::extend (self=0x7fa8a6fedb34, streams=...)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libproc_macro/lib.rs:179
#1  0x00007fa89ce69284 in <proc_macro2::imp::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/unstable.rs:245
#2  0x00007fa89ce6766b in <proc_macro2::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/lib.rs:170
#3  0x00007fa89ce67d9b in <proc_macro2::TokenStream as quote::ext::TokenStreamExt>::append (self=0x7fa8a6fedb30, token=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/ext.rs:41
#4  0x00007fa89ce5fcea in quote::__rt::push_comma (tokens=0x7fa8a6fedb30, span=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/lib.rs:163
#5  0x00007fa89ce47518 in rs_nes_macros::ppu_loop_impl () at rs-nes-macros/src/lib.rs:155
#6  0x00007fa89ce53fb3 in rs_nes_macros::ppu_loop (input=...) at rs-nes-macros/src/lib.rs:490
#7  0x00007fa8a872cdc8 in proc_macro::bridge::client::__run_expand2::{{closure}}::{{closure}} () at src/libproc_macro/bridge/client.rs:409
#8  <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set::{{closure}} () at src/libproc_macro/bridge/scoped_cell.rs:78
#9  <proc_macro::bridge::scoped_cell::ScopedCell<T>>::replace () at src/libproc_macro/bridge/scoped_cell.rs:73
#10 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set () at src/libproc_macro/bridge/scoped_cell.rs:78
#11 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter::{{closure}} () at src/libproc_macro/bridge/client.rs:306
#12 <std::thread::local::LocalKey<T>>::try_with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:299
#13 <std::thread::local::LocalKey<T>>::with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:245
#14 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter () at src/libproc_macro/bridge/client.rs:306
#15 proc_macro::bridge::client::__run_expand2::{{closure}} () at src/libproc_macro/bridge/client.rs:401
#16 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:309
#17 std::panicking::try::do_call () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:297
#18 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#19 0x00007fa8a8732d75 in std::panicking::try () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:276
#20 std::panic::catch_unwind () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:388
#21 proc_macro::bridge::client::__run_expand2 () at src/libproc_macro/bridge/client.rs:400
#22 0x00007fa8abc74d1e in proc_macro::bridge::server::run_server ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#23 0x00007fa8abc384b7 in proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream>>::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#24 0x00007fa8abcfb4a5 in <syntax_ext::proc_macro_impl::AttrProcMacro as syntax::ext::base::AttrProcMacro>::expand ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#25 0x00007fa8a943cb84 in syntax::ext::expand::MacroExpander::expand_invoc ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#26 0x00007fa8a94364de in syntax::ext::expand::MacroExpander::expand_fragment ()
--Type <RET> for more, q to quit, c to continue without paging--
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#27 0x00007fa8a9435624 in syntax::ext::expand::MacroExpander::expand_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#28 0x00007fa8ae3d86a5 in rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}} ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#29 0x00007fa8ae3cf72d in rustc::util::common::time ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#30 0x00007fa8ae436f4e in rustc_driver::driver::phase_2_configure_and_expand ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#31 0x00007fa8ae432842 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#32 0x00007fa8ae38ec21 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#33 0x00007fa8ae39afc6 in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#34 0x00007fa8ae38da6b in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#35 0x00007fa8ae39a9bb in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#36 0x00007fa8ae40d343 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#37 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#38 0x00007fa8ae424ec1 in <F as alloc::boxed::FnBox<A>>::call_box ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#39 0x00007fa8ae06a2ae in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd0c21f6d144d255f () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/liballoc/boxed.rs:744
#40 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:14
#41 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:81
#42 0x00007fa8adf93a9d in start_thread () from /usr/lib/libpthread.so.0
#43 0x00007fa8adeb4b23 in clone () from /usr/lib/libc.so.6

Thread 1 (Thread 0x7fa8a790e380 (LWP 14923)):
#0  0x00007fa8adf94f6d in __pthread_timedjoin_ex () from /usr/lib/libpthread.so.0
#1  0x00007fa8ae06a36d in std::sys::unix::thread::Thread::join () at src/libstd/sys/unix/thread.rs:168
#2  0x00007fa8ae40e1b3 in <std::thread::JoinHandle<T>>::join ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#3  0x00007fa8ae38ba39 in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#4  0x00007fa8ae3998fc in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#5  0x00005589dd3ac453 in std::rt::lang_start::{{closure}} ()
#6  0x00007fa8ae059863 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#7  std::panicking::try::do_call () at src/libstd/panicking.rs:297
#8  0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#9  0x00007fa8ae05a476 in std::panicking::try () at src/libstd/panicking.rs:276
#10 std::panic::catch_unwind () at src/libstd/panic.rs:388
#11 std::rt::lang_start_internal () at src/libstd/rt.rs:48
#12 0x00005589dd3ac442 in main ()

========================================================================

Thread 2 (Thread 0x7fa8a6fff700 (LWP 14924)):
#0  0x00007fa89ce685ea in <proc_macro::TokenStream as core::iter::traits::Extend<proc_macro::TokenStream>>::extend (self=0x7fa8a6fedb34, streams=...)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libproc_macro/lib.rs:179
#1  0x00007fa89ce69284 in <proc_macro2::imp::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/unstable.rs:245
#2  0x00007fa89ce6766b in <proc_macro2::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/lib.rs:170
#3  0x00007fa89ce67d9b in <proc_macro2::TokenStream as quote::ext::TokenStreamExt>::append (self=0x7fa8a6fedb30, token=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/ext.rs:41
#4  0x00007fa89ce5fcea in quote::__rt::push_comma (tokens=0x7fa8a6fedb30, span=...)
    at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/lib.rs:163
#5  0x00007fa89ce47518 in rs_nes_macros::ppu_loop_impl () at rs-nes-macros/src/lib.rs:155
#6  0x00007fa89ce53fb3 in rs_nes_macros::ppu_loop (input=...) at rs-nes-macros/src/lib.rs:490
#7  0x00007fa8a872cdc8 in proc_macro::bridge::client::__run_expand2::{{closure}}::{{closure}} () at src/libproc_macro/bridge/client.rs:409
#8  <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set::{{closure}} () at src/libproc_macro/bridge/scoped_cell.rs:78
#9  <proc_macro::bridge::scoped_cell::ScopedCell<T>>::replace () at src/libproc_macro/bridge/scoped_cell.rs:73
#10 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set () at src/libproc_macro/bridge/scoped_cell.rs:78
#11 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter::{{closure}} () at src/libproc_macro/bridge/client.rs:306
#12 <std::thread::local::LocalKey<T>>::try_with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:299
#13 <std::thread::local::LocalKey<T>>::with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:245
#14 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter () at src/libproc_macro/bridge/client.rs:306
#15 proc_macro::bridge::client::__run_expand2::{{closure}} () at src/libproc_macro/bridge/client.rs:401
#16 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:309
#17 std::panicking::try::do_call () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:297
#18 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#19 0x00007fa8a8732d75 in std::panicking::try () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:276
#20 std::panic::catch_unwind () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:388
#21 proc_macro::bridge::client::__run_expand2 () at src/libproc_macro/bridge/client.rs:400
#22 0x00007fa8abc74d1e in proc_macro::bridge::server::run_server ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#23 0x00007fa8abc384b7 in proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream>>::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#24 0x00007fa8abcfb4a5 in <syntax_ext::proc_macro_impl::AttrProcMacro as syntax::ext::base::AttrProcMacro>::expand ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#25 0x00007fa8a943cb84 in syntax::ext::expand::MacroExpander::expand_invoc ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#26 0x00007fa8a94364de in syntax::ext::expand::MacroExpander::expand_fragment ()
--Type <RET> for more, q to quit, c to continue without paging--c
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#27 0x00007fa8a9435624 in syntax::ext::expand::MacroExpander::expand_crate () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#28 0x00007fa8ae3d86a5 in rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}} () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#29 0x00007fa8ae3cf72d in rustc::util::common::time () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#30 0x00007fa8ae436f4e in rustc_driver::driver::phase_2_configure_and_expand () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#31 0x00007fa8ae432842 in rustc_driver::driver::compile_input () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#32 0x00007fa8ae38ec21 in rustc_driver::run_compiler_with_pool () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#33 0x00007fa8ae39afc6 in <scoped_tls::ScopedKey<T>>::set () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#34 0x00007fa8ae38da6b in rustc_driver::run_compiler () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#35 0x00007fa8ae39a9bb in <scoped_tls::ScopedKey<T>>::set () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#36 0x00007fa8ae40d343 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#37 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#38 0x00007fa8ae424ec1 in <F as alloc::boxed::FnBox<A>>::call_box () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#39 0x00007fa8ae06a2ae in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd0c21f6d144d255f () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/liballoc/boxed.rs:744
#40 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:14
#41 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:81
#42 0x00007fa8adf93a9d in start_thread () from /usr/lib/libpthread.so.0
#43 0x00007fa8adeb4b23 in clone () from /usr/lib/libc.so.6

Thread 1 (Thread 0x7fa8a790e380 (LWP 14923)):
#0  0x00007fa8adf94f6d in __pthread_timedjoin_ex () from /usr/lib/libpthread.so.0
#1  0x00007fa8ae06a36d in std::sys::unix::thread::Thread::join () at src/libstd/sys/unix/thread.rs:168
#2  0x00007fa8ae40e1b3 in <std::thread::JoinHandle<T>>::join () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#3  0x00007fa8ae38ba39 in rustc_driver::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#4  0x00007fa8ae3998fc in rustc_driver::main () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#5  0x00005589dd3ac453 in std::rt::lang_start::{{closure}} ()
#6  0x00007fa8ae059863 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#7  std::panicking::try::do_call () at src/libstd/panicking.rs:297
#8  0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#9  0x00007fa8ae05a476 in std::panicking::try () at src/libstd/panicking.rs:276
#10 std::panic::catch_unwind () at src/libstd/panic.rs:388
#11 std::rt::lang_start_internal () at src/libstd/rt.rs:48
#12 0x00005589dd3ac442 in main ()

===============================================

Thread 2 (Thread 0x7fa8a6fff700 (LWP 14924)):
#0  0x00007fa8ade5cc72 in __memmove_sse2_unaligned_erms () from /usr/lib/libc.so.6
#1  0x00005589dd3d5c14 in _rjem_je_large_ralloc (tsdn=<optimized out>, arena=0x0, extent=0x7fa8a4c16700, usize=<optimized out>, alignment=<optimized out>, 
    zero=<optimized out>, tcache=0x7fa8a6ffe0c0)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/src/large.c:299
#2  0x00005589dd3b87bb in _rjem_je_arena_ralloc (tsdn=0x7fa8a6ffdf00, arena=0x0, ptr=<optimized out>, oldsize=<optimized out>, size=<optimized out>, 
    alignment=0, zero=<optimized out>, tcache=0x7fa8a6ffe0c0)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/src/arena.c:1647
#3  0x00005589dd3af964 in iralloct (tsdn=<optimized out>, ptr=<optimized out>, oldsize=<optimized out>, size=<optimized out>, alignment=0, 
    tcache=0x7fa89c430140, arena=0x0, zero=<optimized out>)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/include/jemalloc/internal/jemalloc_internal_inlines_c.h:190
#4  iralloc (tsd=<optimized out>, ptr=<optimized out>, oldsize=<optimized out>, size=<optimized out>, alignment=0, zero=false)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/include/jemalloc/internal/jemalloc_internal_inlines_c.h:197
#5  realloc (ptr=0x7fa89c931340, size=4910400)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/src/jemalloc.c:2338
#6  0x00007fa8a95cae11 in syntax::tokenstream::TokenStream::from_streams ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#7  0x00007fa8a95cf031 in syntax::tokenstream::TokenStreamBuilder::build ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#8  0x00007fa8abcd344d in std::panicking::try::do_call ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#9  0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#10 0x00007fa8abc76a39 in <proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<S>> as proc_macro::bridge::server::DispatcherTrait>::dispatch () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#11 0x00007fa8abcfc813 in <proc_macro::bridge::closure::Closure<'a, A, R> as core::convert::From<&'a mut F>>::from::call ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#12 0x00007fa8a8752d23 in <proc_macro::bridge::closure::Closure<'a, A, R>>::call () at src/libproc_macro/bridge/closure.rs:30
#13 proc_macro::bridge::client::TokenStreamBuilder::build::{{closure}} () at src/libproc_macro/bridge/client.rs:233
#14 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::with::{{closure}} () at src/libproc_macro/bridge/client.rs:317
#15 proc_macro::bridge::client::BridgeState::with::{{closure}}::{{closure}} () at src/libproc_macro/bridge/client.rs:282
#16 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::replace () at src/libproc_macro/bridge/scoped_cell.rs:73
#17 proc_macro::bridge::client::BridgeState::with::{{closure}} () at src/libproc_macro/bridge/client.rs:280
#18 <std::thread::local::LocalKey<T>>::try_with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:299
--Type <RET> for more, q to quit, c to continue without paging--c
#19 <std::thread::local::LocalKey<T>>::with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:245
#20 proc_macro::bridge::client::BridgeState::with () at src/libproc_macro/bridge/client.rs:279
#21 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::with () at src/libproc_macro/bridge/client.rs:310
#22 proc_macro::bridge::client::TokenStreamBuilder::build () at src/libproc_macro/bridge/client.rs:226
#23 0x00007fa89ce68bec in <proc_macro::TokenStream as core::iter::traits::FromIterator<proc_macro::TokenStream>>::from_iter (streams=...) at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libproc_macro/lib.rs:164
#24 0x00007fa89ce62dd1 in core::iter::iterator::Iterator::collect (self=...) at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libcore/iter/iterator.rs:1468
#25 0x00007fa89ce685ea in <proc_macro::TokenStream as core::iter::traits::Extend<proc_macro::TokenStream>>::extend (self=0x7fa8a6fedb34, streams=...) at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libproc_macro/lib.rs:179
#26 0x00007fa89ce69284 in <proc_macro2::imp::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/unstable.rs:245
#27 0x00007fa89ce6766b in <proc_macro2::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/lib.rs:170
#28 0x00007fa89ce67d9b in <proc_macro2::TokenStream as quote::ext::TokenStreamExt>::append (self=0x7fa8a6fedb30, token=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/ext.rs:41
#29 0x00007fa89ce5fcea in quote::__rt::push_comma (tokens=0x7fa8a6fedb30, span=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/lib.rs:163
#30 0x00007fa89ce47518 in rs_nes_macros::ppu_loop_impl () at rs-nes-macros/src/lib.rs:155
#31 0x00007fa89ce53fb3 in rs_nes_macros::ppu_loop (input=...) at rs-nes-macros/src/lib.rs:490
#32 0x00007fa8a872cdc8 in proc_macro::bridge::client::__run_expand2::{{closure}}::{{closure}} () at src/libproc_macro/bridge/client.rs:409
#33 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set::{{closure}} () at src/libproc_macro/bridge/scoped_cell.rs:78
#34 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::replace () at src/libproc_macro/bridge/scoped_cell.rs:73
#35 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set () at src/libproc_macro/bridge/scoped_cell.rs:78
#36 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter::{{closure}} () at src/libproc_macro/bridge/client.rs:306
#37 <std::thread::local::LocalKey<T>>::try_with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:299
#38 <std::thread::local::LocalKey<T>>::with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:245
#39 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter () at src/libproc_macro/bridge/client.rs:306
#40 proc_macro::bridge::client::__run_expand2::{{closure}} () at src/libproc_macro/bridge/client.rs:401
#41 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:309
#42 std::panicking::try::do_call () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:297
#43 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#44 0x00007fa8a8732d75 in std::panicking::try () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:276
#45 std::panic::catch_unwind () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:388
#46 proc_macro::bridge::client::__run_expand2 () at src/libproc_macro/bridge/client.rs:400
#47 0x00007fa8abc74d1e in proc_macro::bridge::server::run_server () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#48 0x00007fa8abc384b7 in proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream>>::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#49 0x00007fa8abcfb4a5 in <syntax_ext::proc_macro_impl::AttrProcMacro as syntax::ext::base::AttrProcMacro>::expand () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#50 0x00007fa8a943cb84 in syntax::ext::expand::MacroExpander::expand_invoc () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#51 0x00007fa8a94364de in syntax::ext::expand::MacroExpander::expand_fragment () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#52 0x00007fa8a9435624 in syntax::ext::expand::MacroExpander::expand_crate () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#53 0x00007fa8ae3d86a5 in rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}} () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#54 0x00007fa8ae3cf72d in rustc::util::common::time () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#55 0x00007fa8ae436f4e in rustc_driver::driver::phase_2_configure_and_expand () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#56 0x00007fa8ae432842 in rustc_driver::driver::compile_input () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#57 0x00007fa8ae38ec21 in rustc_driver::run_compiler_with_pool () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#58 0x00007fa8ae39afc6 in <scoped_tls::ScopedKey<T>>::set () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#59 0x00007fa8ae38da6b in rustc_driver::run_compiler () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#60 0x00007fa8ae39a9bb in <scoped_tls::ScopedKey<T>>::set () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#61 0x00007fa8ae40d343 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#62 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#63 0x00007fa8ae424ec1 in <F as alloc::boxed::FnBox<A>>::call_box () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#64 0x00007fa8ae06a2ae in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd0c21f6d144d255f () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/liballoc/boxed.rs:744
#65 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:14
#66 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:81
#67 0x00007fa8adf93a9d in start_thread () from /usr/lib/libpthread.so.0
#68 0x00007fa8adeb4b23 in clone () from /usr/lib/libc.so.6

Thread 1 (Thread 0x7fa8a790e380 (LWP 14923)):
#0  0x00007fa8adf94f6d in __pthread_timedjoin_ex () from /usr/lib/libpthread.so.0
#1  0x00007fa8ae06a36d in std::sys::unix::thread::Thread::join () at src/libstd/sys/unix/thread.rs:168
#2  0x00007fa8ae40e1b3 in <std::thread::JoinHandle<T>>::join () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#3  0x00007fa8ae38ba39 in rustc_driver::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#4  0x00007fa8ae3998fc in rustc_driver::main () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#5  0x00005589dd3ac453 in std::rt::lang_start::{{closure}} ()
#6  0x00007fa8ae059863 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#7  std::panicking::try::do_call () at src/libstd/panicking.rs:297
#8  0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#9  0x00007fa8ae05a476 in std::panicking::try () at src/libstd/panicking.rs:276
#10 std::panic::catch_unwind () at src/libstd/panic.rs:388
#11 std::rt::lang_start_internal () at src/libstd/rt.rs:48

===============================================

Thread 2 (Thread 0x7fa8a6fff700 (LWP 14924)):
#0  0x00007fa8ade5cc72 in __memmove_sse2_unaligned_erms () from /usr/lib/libc.so.6
#1  0x00005589dd3d5c14 in _rjem_je_large_ralloc (tsdn=<optimized out>, arena=0x0, extent=0x7fa8a4c16700, usize=<optimized out>, alignment=<optimized out>, 
    zero=<optimized out>, tcache=0x7fa8a6ffe0c0)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/src/large.c:299
#2  0x00005589dd3b87bb in _rjem_je_arena_ralloc (tsdn=0x7fa8a6ffdf00, arena=0x0, ptr=<optimized out>, oldsize=<optimized out>, size=<optimized out>, 
    alignment=0, zero=<optimized out>, tcache=0x7fa8a6ffe0c0)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/src/arena.c:1647
#3  0x00005589dd3af964 in iralloct (tsdn=<optimized out>, ptr=<optimized out>, oldsize=<optimized out>, size=<optimized out>, alignment=0, 
    tcache=0x7fa89c430140, arena=0x0, zero=<optimized out>)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/include/jemalloc/internal/jemalloc_internal_inlines_c.h:190
#4  iralloc (tsd=<optimized out>, ptr=<optimized out>, oldsize=<optimized out>, size=<optimized out>, alignment=0, zero=false)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/include/jemalloc/internal/jemalloc_internal_inlines_c.h:197
#5  realloc (ptr=0x7fa89c931340, size=4910400)
    at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/jemalloc-sys-52018b331c920d2b/out/jemalloc/src/jemalloc.c:2338
#6  0x00007fa8a95cae11 in syntax::tokenstream::TokenStream::from_streams ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#7  0x00007fa8a95cf031 in syntax::tokenstream::TokenStreamBuilder::build ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#8  0x00007fa8abcd344d in std::panicking::try::do_call ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#9  0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#10 0x00007fa8abc76a39 in <proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<S>> as proc_macro::bridge::server::DispatcherTrait>::dispatch () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#11 0x00007fa8abcfc813 in <proc_macro::bridge::closure::Closure<'a, A, R> as core::convert::From<&'a mut F>>::from::call ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#12 0x00007fa8a8752d23 in <proc_macro::bridge::closure::Closure<'a, A, R>>::call () at src/libproc_macro/bridge/closure.rs:30
#13 proc_macro::bridge::client::TokenStreamBuilder::build::{{closure}} () at src/libproc_macro/bridge/client.rs:233
#14 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::with::{{closure}} () at src/libproc_macro/bridge/client.rs:317
#15 proc_macro::bridge::client::BridgeState::with::{{closure}}::{{closure}} () at src/libproc_macro/bridge/client.rs:282
#16 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::replace () at src/libproc_macro/bridge/scoped_cell.rs:73
#17 proc_macro::bridge::client::BridgeState::with::{{closure}} () at src/libproc_macro/bridge/client.rs:280
#18 <std::thread::local::LocalKey<T>>::try_with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:299
--Type <RET> for more, q to quit, c to continue without paging--c
#19 <std::thread::local::LocalKey<T>>::with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:245
#20 proc_macro::bridge::client::BridgeState::with () at src/libproc_macro/bridge/client.rs:279
#21 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::with () at src/libproc_macro/bridge/client.rs:310
#22 proc_macro::bridge::client::TokenStreamBuilder::build () at src/libproc_macro/bridge/client.rs:226
#23 0x00007fa89ce68bec in <proc_macro::TokenStream as core::iter::traits::FromIterator<proc_macro::TokenStream>>::from_iter (streams=...) at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libproc_macro/lib.rs:164
#24 0x00007fa89ce62dd1 in core::iter::iterator::Iterator::collect (self=...) at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libcore/iter/iterator.rs:1468
#25 0x00007fa89ce685ea in <proc_macro::TokenStream as core::iter::traits::Extend<proc_macro::TokenStream>>::extend (self=0x7fa8a6fedb34, streams=...) at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libproc_macro/lib.rs:179
#26 0x00007fa89ce69284 in <proc_macro2::imp::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/unstable.rs:245
#27 0x00007fa89ce6766b in <proc_macro2::TokenStream as core::iter::traits::Extend<proc_macro2::TokenTree>>::extend (self=0x7fa8a6fedb30, streams=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.24/src/lib.rs:170
#28 0x00007fa89ce67d9b in <proc_macro2::TokenStream as quote::ext::TokenStreamExt>::append (self=0x7fa8a6fedb30, token=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/ext.rs:41
#29 0x00007fa89ce5fcea in quote::__rt::push_comma (tokens=0x7fa8a6fedb30, span=...) at /home/lampam/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-0.6.10/src/lib.rs:163
#30 0x00007fa89ce47518 in rs_nes_macros::ppu_loop_impl () at rs-nes-macros/src/lib.rs:155
#31 0x00007fa89ce53fb3 in rs_nes_macros::ppu_loop (input=...) at rs-nes-macros/src/lib.rs:490
#32 0x00007fa8a872cdc8 in proc_macro::bridge::client::__run_expand2::{{closure}}::{{closure}} () at src/libproc_macro/bridge/client.rs:409
#33 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set::{{closure}} () at src/libproc_macro/bridge/scoped_cell.rs:78
#34 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::replace () at src/libproc_macro/bridge/scoped_cell.rs:73
#35 <proc_macro::bridge::scoped_cell::ScopedCell<T>>::set () at src/libproc_macro/bridge/scoped_cell.rs:78
#36 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter::{{closure}} () at src/libproc_macro/bridge/client.rs:306
#37 <std::thread::local::LocalKey<T>>::try_with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:299
#38 <std::thread::local::LocalKey<T>>::with () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/thread/local.rs:245
#39 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge<'_>>::enter () at src/libproc_macro/bridge/client.rs:306
#40 proc_macro::bridge::client::__run_expand2::{{closure}} () at src/libproc_macro/bridge/client.rs:401
#41 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:309
#42 std::panicking::try::do_call () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:297
#43 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#44 0x00007fa8a8732d75 in std::panicking::try () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panicking.rs:276
#45 std::panic::catch_unwind () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/libstd/panic.rs:388
#46 proc_macro::bridge::client::__run_expand2 () at src/libproc_macro/bridge/client.rs:400
#47 0x00007fa8abc74d1e in proc_macro::bridge::server::run_server () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#48 0x00007fa8abc384b7 in proc_macro::bridge::server::<impl proc_macro::bridge::client::Client<fn(proc_macro::TokenStream, proc_macro::TokenStream) -> proc_macro::TokenStream>>::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#49 0x00007fa8abcfb4a5 in <syntax_ext::proc_macro_impl::AttrProcMacro as syntax::ext::base::AttrProcMacro>::expand () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_ext-065cea48e300124c.so
#50 0x00007fa8a943cb84 in syntax::ext::expand::MacroExpander::expand_invoc () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#51 0x00007fa8a94364de in syntax::ext::expand::MacroExpander::expand_fragment () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#52 0x00007fa8a9435624 in syntax::ext::expand::MacroExpander::expand_crate () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-2e8dd76d87693318.so
#53 0x00007fa8ae3d86a5 in rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}} () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#54 0x00007fa8ae3cf72d in rustc::util::common::time () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#55 0x00007fa8ae436f4e in rustc_driver::driver::phase_2_configure_and_expand () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#56 0x00007fa8ae432842 in rustc_driver::driver::compile_input () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#57 0x00007fa8ae38ec21 in rustc_driver::run_compiler_with_pool () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#58 0x00007fa8ae39afc6 in <scoped_tls::ScopedKey<T>>::set () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#59 0x00007fa8ae38da6b in rustc_driver::run_compiler () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#60 0x00007fa8ae39a9bb in <scoped_tls::ScopedKey<T>>::set () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#61 0x00007fa8ae40d343 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#62 0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#63 0x00007fa8ae424ec1 in <F as alloc::boxed::FnBox<A>>::call_box () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#64 0x00007fa8ae06a2ae in _$LT$alloc..boxed..Box$LT$$LP$dyn$u20$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$RP$$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::hd0c21f6d144d255f () at /rustc/daa53a52a2667533d5fe59bfcc5b8614b79c3d31/src/liballoc/boxed.rs:744
#65 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:14
#66 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:81
#67 0x00007fa8adf93a9d in start_thread () from /usr/lib/libpthread.so.0
#68 0x00007fa8adeb4b23 in clone () from /usr/lib/libc.so.6

Thread 1 (Thread 0x7fa8a790e380 (LWP 14923)):
#0  0x00007fa8adf94f6d in __pthread_timedjoin_ex () from /usr/lib/libpthread.so.0
#1  0x00007fa8ae06a36d in std::sys::unix::thread::Thread::join () at src/libstd/sys/unix/thread.rs:168
#2  0x00007fa8ae40e1b3 in <std::thread::JoinHandle<T>>::join () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#3  0x00007fa8ae38ba39 in rustc_driver::run () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#4  0x00007fa8ae3998fc in rustc_driver::main () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-c5efcb7334989aff.so
#5  0x00005589dd3ac453 in std::rt::lang_start::{{closure}} ()
#6  0x00007fa8ae059863 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#7  std::panicking::try::do_call () at src/libstd/panicking.rs:297
#8  0x00007fa8ae06b4ea in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:92
#9  0x00007fa8ae05a476 in std::panicking::try () at src/libstd/panicking.rs:276
#10 std::panic::catch_unwind () at src/libstd/panic.rs:388
#11 std::rt::lang_start_internal () at src/libstd/rt.rs:48
#12 0x00005589dd3ac442 in main ()
