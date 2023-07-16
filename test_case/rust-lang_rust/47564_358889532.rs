
#0  0x00007fd72af105c8 in __memcmp_avx2_movbe () from /usr/lib/libc.so.6
#1  0x00007fd72a0a719c in <[u8] as core::slice::SliceOrd<u8>>::compare (self=..., 
    other=...) at /home/nathan/Projects/rust/src/libcore/slice/mod.rs:2887
#2  core::slice::<impl core::cmp::Ord for [T]>::cmp (self=..., other=...)
    at /home/nathan/Projects/rust/src/libcore/slice/mod.rs:2759
#3  core::str::traits::<impl core::cmp::Ord for str>::cmp (self=..., other=...)
    at /home/nathan/Projects/rust/src/libcore/str/mod.rs:1597
#4  core::str::traits::<impl core::cmp::PartialOrd for str>::partial_cmp (
    self=..., other=...) at /home/nathan/Projects/rust/src/libcore/str/mod.rs:1625
#5  core::cmp::PartialOrd::lt (self=..., other=...)
    at /home/nathan/Projects/rust/src/libcore/cmp.rs:645
#6  core::cmp::impls::<impl core::cmp::PartialOrd<&'b B> for &'a A>::lt (
    self=<optimized out>, other=<optimized out>)
    at /home/nathan/Projects/rust/src/libcore/cmp.rs:923
#7  <syntax_pos::symbol::InternedString as core::cmp::PartialOrd>::lt (
    self=<optimized out>, __arg_0=<optimized out>)
    at /home/nathan/Projects/rust/src/libsyntax_pos/symbol.rs:352
#8  core::tuple::<impl core::cmp::PartialOrd for (A, B)>::lt (
    other=<optimized out>, self=<optimized out>)
    at /home/nathan/Projects/rust/src/libcore/tuple.rs:92
#9  <[T] as core::slice::SliceExt>::sort_unstable_by_key::{{closure}} (
    a=<optimized out>, b=<optimized out>)
    at /home/nathan/Projects/rust/src/libcore/slice/mod.rs:754
#10 core::slice::sort::partition_in_blocks (v=..., pivot=<optimized out>, 
    is_less=<optimized out>)
    at /home/nathan/Projects/rust/src/libcore/slice/sort.rs:300
#11 core::slice::sort::partition (v=..., pivot=..., is_less=<optimized out>)
    at /home/nathan/Projects/rust/src/libcore/slice/sort.rs:422
#12 core::slice::sort::recurse (v=..., is_less=0x7fd7205f7120, pred=..., limit=10)
    at /home/nathan/Projects/rust/src/libcore/slice/sort.rs:667
#13 0x00007fd72a08fcaa in core::slice::sort::quicksort (v=..., is_less=...)
    at /home/nathan/Projects/rust/src/libcore/slice/sort.rs:702
#14 <[T] as core::slice::SliceExt>::sort_unstable_by_key (self=..., f=...)
    at /home/nathan/Projects/rust/src/libcore/slice/mod.rs:754
#15 alloc::slice::<impl [T]>::sort_unstable_by_key (self=..., f=...)
    at /home/nathan/Projects/rust/src/liballoc/slice.rs:1434
#16 rustc_resolve::ModuleData::for_each_child_stable (self=<optimized out>, f=...)
    at librustc_resolve/lib.rs:987
#17 rustc_resolve::Resolver::lookup_import_candidates (lookup_name=..., 
    namespace=rustc_resolve::Namespace::TypeNS, self=<optimized out>, 
    filter_fn=...) at librustc_resolve/lib.rs:3596
#18 rustc_resolve::Resolver::resolve_path (self=<optimized out>, path=..., 
    opt_ns=..., 
    record_used=<error reading variable: access outside bounds of object referenced via synthetic pointer>, path_span=...) at librustc_resolve/lib.rs:3092
#19 0x00007fd72a0c8542 in rustc_resolve::resolve_imports::ImportResolver::resolve_import (directive=0x7fd6f6a8ed70, self=<optimized out>)
    at librustc_resolve/resolve_imports.rs:538
#20 rustc_resolve::resolve_imports::ImportResolver::resolve_imports (
    self=0x7fd7205f7378) at librustc_resolve/resolve_imports.rs:468
#21 0x00007fd72a068ba5 in rustc_resolve::macros::<impl syntax::ext::base::Resolver ---Type <return> to continue, or q <return> to quit---
for rustc_resolve::Resolver<'a>>::resolve_imports (self=<optimized out>)
    at librustc_resolve/macros.rs:206
#22 0x00007fd7260749f4 in syntax::ext::expand::MacroExpander::resolve_imports (
    self=0x7fd7205f81c0) at libsyntax/ext/expand.rs:388
#23 syntax::ext::expand::MacroExpander::expand (self=<optimized out>, 
    expansion=...) at libsyntax/ext/expand.rs:281
#24 0x00007fd726072c83 in syntax::ext::expand::MacroExpander::expand_crate (
    self=0x7fd7205f81c0, krate=...) at libsyntax/ext/expand.rs:245
#25 0x00007fd72b5a3039 in rustc_driver::driver::phase_2_configure_and_expand::{{closure}} () at librustc_driver/driver.rs:780
#26 0x00007fd72b589650 in rustc::util::common::time (do_it=<optimized out>, 
    what=..., f=...) at /home/nathan/Projects/rust/src/librustc/util/common.rs:120
#27 0x00007fd72b62dad8 in rustc_driver::driver::phase_2_configure_and_expand (
    sess=<optimized out>, cstore=<optimized out>, krate=..., registry=..., 
    crate_name=..., addl_plugins=..., make_glob_map=<optimized out>, 
    after_expand=...) at librustc_driver/driver.rs:739
#28 rustc_driver::driver::compile_input (sess=<optimized out>, 
    cstore=0x7fd7205fa801, input_path=<optimized out>, input=<optimized out>, 
    outdir=<optimized out>, output=<optimized out>, addl_plugins=..., 
    control=<optimized out>) at librustc_driver/driver.rs:162
#29 0x00007fd72b6366ad in rustc_driver::run_compiler (args=..., callbacks=..., 
    file_loader=..., emitter_dest=...) at librustc_driver/lib.rs:248
#30 0x00007fd72b534ee6 in rustc_driver::main::{{closure}} ()
    at librustc_driver/lib.rs:1305
#31 rustc_driver::run::{{closure}} () at librustc_driver/lib.rs:132
#32 rustc_driver::monitor::{{closure}} () at librustc_driver/lib.rs:1212
#33 std::sys_common::backtrace::__rust_begin_short_backtrace (f=...)
    at /home/nathan/Projects/rust/src/libstd/sys_common/backtrace.rs:133
#34 0x00007fd72b209b9f in __rust_maybe_catch_panic (f=0x49, 
    data=0x7fd71f337ce0 "IsCallable\000", data_ptr=0x7fd7205fe9f8, 
    vtable_ptr=0x7fd7205fe9f0) at libpanic_unwind/lib.rs:102
#35 0x00007fd72b583e37 in std::panicking::try (f=...)
    at /home/nathan/Projects/rust/src/libstd/panicking.rs:458
#36 std::panic::catch_unwind (f=...)
    at /home/nathan/Projects/rust/src/libstd/panic.rs:358
#37 std::thread::Builder::spawn::{{closure}} ()
    at /home/nathan/Projects/rust/src/libstd/thread/mod.rs:405
#38 <F as alloc::boxed::FnBox<A>>::call_box (self=0x7fd720716b60, 
    args=<optimized out>) at /home/nathan/Projects/rust/src/liballoc/boxed.rs:817
#39 0x00007fd72b1de438 in _$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h2c212c9eabf9bacb (
    self=<error reading variable: access outside bounds of object referenced via synthetic pointer>, args=<optimized out>)
    at /home/nathan/Projects/rust/src/liballoc/boxed.rs:827
#40 std::sys_common::thread::start_thread (main=0x7fd72062b040 "`kq \327\177\000")
    at libstd/sys_common/thread.rs:24
#41 0x00007fd72b1e7249 in std::sys::unix::thread::Thread::new::thread_start (
    main=0x7fd71f337ce0) at libstd/sys/unix/thread.rs:90
#42 0x00007fd7251d808c in start_thread () from /usr/lib/libpthread.so.0
#43 0x00007fd72aeb2e1f in clone () from /usr/lib/libc.so.6
