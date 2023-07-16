
#0  0x00007f7b7a13d42e in llvm::orc::NullLegacyResolver::findSymbolInLogicalDylib(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) () from /usr/lib/../lib/libLLVM-11.so
#1  0x00007f7b964702de in call_init.part () from /lib64/ld-linux-x86-64.so.2
#2  0x00007f7b964703c8 in _dl_init () from /lib64/ld-linux-x86-64.so.2
#3  0x00007f7b9168a0e5 in _dl_catch_exception () from /usr/lib/libc.so.6
#4  0x00007f7b96474705 in dl_open_worker () from /lib64/ld-linux-x86-64.so.2
#5  0x00007f7b9168a088 in _dl_catch_exception () from /usr/lib/libc.so.6
#6  0x00007f7b96473f3e in _dl_open () from /lib64/ld-linux-x86-64.so.2
#7  0x00007f7b9171b34c in ?? () from /usr/lib/libdl.so.2
#8  0x00007f7b9168a088 in _dl_catch_exception () from /usr/lib/libc.so.6
#9  0x00007f7b9168a153 in _dl_catch_error () from /usr/lib/libc.so.6
#10 0x00007f7b9171bb89 in ?? () from /usr/lib/libdl.so.2
#11 0x00007f7b9171b3d8 in dlopen () from /usr/lib/libdl.so.2
#12 0x00007f7b84e1fb38 in libloading::os::unix::{{impl}}::open::{{closure}}<&std::path::PathBuf> () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/libloading-0.6.6/src/os/unix/mod.rs:149
#13 0x00007f7b84e0f977 in libloading::os::unix::with_dlerror<libloading::os::unix::Library,closure-1> (wrap=0x7f7b84ddf950 <core::ops::function::FnOnce::call_once<closure-0,(libloading::error::DlDescription)>>, closure=...)
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/libloading-0.6.6/src/os/unix/mod.rs:55
#14 0x00007f7b84e1f9bc in libloading::os::unix::Library::open<&std::path::PathBuf> (filename=..., flags=2) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/libloading-0.6.6/src/os/unix/mod.rs:147
#15 0x00007f7b84e1f795 in libloading::os::unix::Library::new<&std::path::PathBuf> (filename=0x7f7b8bfd6448) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/libloading-0.6.6/src/os/unix/mod.rs:111
#16 0x00007f7b84ec1173 in libloading::Library::new<&std::path::PathBuf> (filename=0x7f7b8bfd6448) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/libloading-0.6.6/src/lib.rs:131
#17 0x00007f7b84d83120 in clang_sys::load_manually () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/clang-sys-1.0.3/src/link.rs:198
#18 0x00007f7b84d84b43 in clang_sys::load () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/clang-sys-1.0.3/src/link.rs:224
#19 0x00007f7b849e469f in autocxx_bindgen::ensure_libclang_is_loaded::{{impl}}::deref::__static_ref_initialize () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/autocxx-bindgen-0.56.1/src/lib.rs:1922
#20 core::ops::function::FnOnce::call_once<fn() -> alloc::sync::Arc<clang_sys::SharedLibrary>,()> () at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
#21 0x00007f7b8491cb07 in lazy_static::lazy::{{impl}}::get::{{closure}}<alloc::sync::Arc<clang_sys::SharedLibrary>,fn() -> alloc::sync::Arc<clang_sys::SharedLibrary>> ()
    at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/inline_lazy.rs:31
#22 0x00007f7b849e35ef in std::sync::once::{{impl}}::call_once::{{closure}}<closure-0> () at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/once.rs:261
#23 0x00007f7b9180dff2 in std::sync::once::Once::call_inner () at library/std/src/sync/once.rs:419
#24 0x00007f7b849e3445 in std::sync::once::Once::call_once<closure-0> (self=0x7f7b856cf218 <<autocxx_bindgen::ensure_libclang_is_loaded::LIBCLANG as core::ops::deref::Deref>::deref::__stability::LAZY+8>, f=...)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/once.rs:261
#25 0x00007f7b84a0ad29 in lazy_static::lazy::Lazy<alloc::sync::Arc<clang_sys::SharedLibrary>>::get<alloc::sync::Arc<clang_sys::SharedLibrary>,fn() -> alloc::sync::Arc<clang_sys::SharedLibrary>> (
    self=0x7f7b856cf210 <<autocxx_bindgen::ensure_libclang_is_loaded::LIBCLANG as core::ops::deref::Deref>::deref::__stability::LAZY>, f=<optimized out>) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/inline_lazy.rs:30
#26 autocxx_bindgen::ensure_libclang_is_loaded::{{impl}}::deref::__stability () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/lib.rs:142
#27 autocxx_bindgen::ensure_libclang_is_loaded::{{impl}}::deref (self=0x7f7b852c497a) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/lib.rs:144
#28 0x00007f7b84a01208 in autocxx_bindgen::ensure_libclang_is_loaded () at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/autocxx-bindgen-0.56.1/src/lib.rs:1930
#29 0x00007f7b84a017cd in autocxx_bindgen::Bindings::generate (options=...) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/autocxx-bindgen-0.56.1/src/lib.rs:1988
#30 0x00007f7b849fe106 in autocxx_bindgen::Builder::generate (self=...) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/autocxx-bindgen-0.56.1/src/lib.rs:1335
#31 0x00007f7b8459d8e7 in autocxx_engine::IncludeCpp::generate (self=0x7f7b8bfea5e8) at engine/src/lib.rs:377
#32 0x00007f7b8457f519 in autocxx_macro::include_cpp_impl::{{closure}} () at macro/src/lib.rs:25
#33 0x00007f7b8457f8af in std::panic::{{impl}}::call_once<proc_macro::TokenStream,closure-0> (self=..., _args=()) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:322
#34 0x00007f7b84583bb0 in std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure-0>,proc_macro::TokenStream> (data=0x7f7b8bfeaf10) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:381
#35 0x00007f7b84583d3d in __rust_try () from /tmp/autocxx/target/debug/deps/libautocxx_macro-4e0e6644536281ef.so
#36 0x00007f7b84583906 in std::panicking::try<proc_macro::TokenStream,std::panic::AssertUnwindSafe<closure-0>> (f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:345
#37 0x00007f7b8457f938 in std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure-0>,proc_macro::TokenStream> (f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:396
#38 0x00007f7b8457d4d0 in proc_macro_error::entry_point<std::panic::AssertUnwindSafe<closure-0>> (f=..., proc_macro_hack=false) at ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro-error-1.0.4/src/lib.rs:432
#39 0x00007f7b8457df0b in autocxx_macro::include_cpp_impl (input=...) at macro/src/lib.rs:21
#40 0x00007f7b8457f015 in core::ops::function::FnOnce::call_once<fn(proc_macro::TokenStream) -> proc_macro::TokenStream,(proc_macro::TokenStream)> ()
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
#41 0x00007f7b84584d06 in proc_macro::bridge::client::{{impl}}::expand1::run::{{closure}}<fn(proc_macro::TokenStream) -> proc_macro::TokenStream> (input=...)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:411
#42 0x00007f7b84584363 in proc_macro::bridge::client::run_client::{{closure}}::{{closure}}<proc_macro::bridge::client::TokenStream,proc_macro::bridge::client::TokenStream,closure-0> ()
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:377
#43 0x00007f7b84580888 in proc_macro::bridge::scoped_cell::{{impl}}::set::{{closure}}<proc_macro::bridge::client::BridgeStateL,(),closure-0> ()
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:81
#44 0x00007f7b84580a9d in proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>::replace<proc_macro::bridge::client::BridgeStateL,(),closure-0> (self=0x7f7b8bffdb68, replacement=..., f=...)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:76
#45 0x00007f7b84580861 in proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>::set<proc_macro::bridge::client::BridgeStateL,(),closure-0> (self=0x7f7b8bffdb68, value=..., f=...)
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:81
#46 0x00007f7b84584acf in proc_macro::bridge::client::{{impl}}::enter::{{closure}}<(),closure-0> (state=0x7f7b8bffdb68) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:325
#47 0x00007f7b84582acd in std::thread::local::LocalKey<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>::try_with<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>,closure-1,()> (
    self=0x7f7b85614aa0, f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:272
#48 0x00007f7b8458262d in std::thread::local::LocalKey<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>::with<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>,closure-1,()> (
    self=0x7f7b85614aa0, f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:248
#49 0x00007f7b8457e72d in proc_macro::bridge::Bridge::enter<(),closure-0> (self=..., f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:325
#50 0x00007f7b84584254 in proc_macro::bridge::client::run_client::{{closure}}<proc_macro::bridge::client::TokenStream,proc_macro::bridge::client::TokenStream,closure-0> ()
    at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:370
#51 0x00007f7b8457f902 in std::panic::{{impl}}::call_once<(),closure-0> (self=..., _args=()) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:322
#52 0x00007f7b84583b60 in std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure-0>,()> (data=0x7f7b8bfebac8) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:381
#53 0x00007f7b84583d3d in __rust_try () from /tmp/autocxx/target/debug/deps/libautocxx_macro-4e0e6644536281ef.so
#54 0x00007f7b84583a80 in std::panicking::try<(),std::panic::AssertUnwindSafe<closure-0>> (f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:345
#55 0x00007f7b8457f982 in std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure-0>,()> (f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:396
#56 0x00007f7b845840ac in proc_macro::bridge::client::run_client<proc_macro::bridge::client::TokenStream,proc_macro::bridge::client::TokenStream,closure-0> (bridge=..., f=...) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:369
#57 0x00007f7b84584cc1 in proc_macro::bridge::client::{{impl}}::expand1::run<fn(proc_macro::TokenStream) -> proc_macro::TokenStream> (bridge=..., f=0x7f7b8457dee0 <autocxx_macro::include_cpp_impl>) at ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:411
#58 0x00007f7b9414deb4 in proc_macro::bridge::server::run_server () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#59 0x00007f7b940ba011 in <rustc_expand::proc_macro::BangProcMacro as rustc_expand::base::ProcMacro>::expand () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#60 0x00007f7b9408132b in rustc_expand::expand::MacroExpander::fully_expand_fragment () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#61 0x00007f7b9407f6e8 in rustc_expand::expand::MacroExpander::expand_crate () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#62 0x00007f7b922e3a36 in rustc_session::utils::<impl rustc_session::session::Session>::time () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#63 0x00007f7b9232fc52 in rustc_interface::passes::configure_and_expand_inner () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#64 0x00007f7b922bf475 in rustc_interface::passes::configure_and_expand::_$u7b$$u7b$closure$u7d$$u7d$::h2410401d2ea7094a () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#65 0x00007f7b922b60cf in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#66 0x00007f7b9232ebe6 in rustc_interface::passes::configure_and_expand () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#67 0x00007f7b92358c2b in rustc_interface::queries::Queries::expansion () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#68 0x00007f7b92105c85 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#69 0x00007f7b920f3cae in rustc_span::with_source_map () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#70 0x00007f7b92106f52 in rustc_interface::interface::create_compiler_and_run () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#71 0x00007f7b920f485a in rustc_span::with_session_globals () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#72 0x00007f7b9210d04f in std::sys_common::backtrace::__rust_begin_short_backtrace () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#73 0x00007f7b920747f8 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-74849affecce5bb0.so
#74 0x00007f7b9182352a in alloc::boxed::{{impl}}::call_once<(),FnOnce<()>,alloc::alloc::Global> () at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/alloc/src/boxed.rs:1307
#75 alloc::boxed::{{impl}}::call_once<(),alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>,alloc::alloc::Global> () at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/alloc/src/boxed.rs:1307
#76 std::sys::unix::thread::{{impl}}::new::thread_start () at library/std/src/sys/unix/thread.rs:71
#77 0x00007f7b917343e9 in start_thread () from /usr/lib/libpthread.so.0
#78 0x00007f7b9164f293 in clone () from /usr/lib/libc.so.6
