
#0  atomic_load_p (a=0x27478, mo=atomic_memory_order_relaxed) at ../jemalloc/include/jemalloc/internal/atomic.h:55
#1  rtree_leaf_elm_bits_read (tsdn=<optimized out>, rtree=<optimized out>, elm=0x27478, dependent=true) at ../jemalloc/include/jemalloc/internal/rtree.h:175
#2  rtree_szind_slab_read (tsdn=0x7fd63c1fde40, rtree=<optimized out>, rtree_ctx=0x7fd63c1fde68, key=140558255847381, dependent=true, r_szind=<optimized out>, r_slab=<optimized out>)
    at ../jemalloc/include/jemalloc/internal/rtree.h:464
#3  ifree (tsd=<optimized out>, ptr=0x7fd644e8f7d5 <proc_macro::Punct::as_char+37>, tcache=0x7fd63c1fe000, slow_path=false) at ../jemalloc/src/jemalloc.c:2206
#4  free (ptr=0x7fd644e8f7d5 <proc_macro::Punct::as_char+37>) at ../jemalloc/src/jemalloc.c:2393
#5  0x00007fd643192418 in core::ptr::drop_in_place<proc_macro::bridge::buffer::Buffer<u8>> () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#6  0x00007fd6442761cf in <proc_macro::bridge::server::Dispatcher<proc_macro::bridge::server::MarkedTypes<S>> as proc_macro::bridge::server::DispatcherTrait>::dispatch ()
   from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#7  0x00007fd644282f33 in _$LT$proc_macro..bridge..closure..Closure$LT$A$C$R$GT$$u20$as$u20$core..convert..From$LT$$RF$mut$u20$F$GT$$GT$::from::call::h2b7103ad48cadfdc ()
   from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#8  0x00007fd644e8fc94 in proc_macro::bridge::closure::Closure::call<proc_macro::bridge::buffer::Buffer<u8>,proc_macro::bridge::buffer::Buffer<u8>> ()
    at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/closure.rs:27
#9  proc_macro::bridge::client::{{impl}}::as_char::{{closure}} () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:244
#10 proc_macro::bridge::client::{{impl}}::with::{{closure}}<char,closure-0> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:336
#11 proc_macro::bridge::client::{{impl}}::with::{{closure}}::{{closure}}<char,closure-0> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:293
#12 proc_macro::bridge::scoped_cell::ScopedCell::replace<proc_macro::bridge::client::BridgeStateL,char,closure-0> ()
    at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/scoped_cell.rs:75
#13 proc_macro::bridge::client::{{impl}}::with::{{closure}}<char,closure-0> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:291
#14 std::thread::local::LocalKey::try_with<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>,closure-0,char> ()
    at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/thread/local.rs:272
#15 std::thread::local::LocalKey::with<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>,closure-0,char> ()
    at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/std/src/thread/local.rs:248
#16 proc_macro::bridge::client::BridgeState::with<char,closure-0> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:290
#17 proc_macro::bridge::Bridge::with<char,closure-0> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:329
#18 proc_macro::bridge::client::Punct::as_char () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/bridge/client.rs:237
--Type <RET> for more, q to quit, c to continue without paging--c
#19 proc_macro::Punct::as_char () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/proc_macro/src/lib.rs:791
#20 0x00007fd632ffd1cf in proc_macro2::imp::{{impl}}::next (self=0x7fd63c1e8e08) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.26/src/wrapper.rs:332
#21 0x00007fd63300bf37 in proc_macro2::token_stream::{{impl}}::next (self=0x7fd63c1e8e08) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.26/src/lib.rs:1240
#22 0x00007fd632f9fb07 in syn::buffer::TokenBuffer::inner_new (stream=..., up=0x7fd632854900) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/buffer.rs:53
#23 0x00007fd632fa0203 in syn::buffer::TokenBuffer::inner_new (stream=..., up=0x7fd63b596b80) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/buffer.rs:89
#24 0x00007fd632fa0203 in syn::buffer::TokenBuffer::inner_new (stream=..., up=0x0) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/buffer.rs:89
#25 0x00007fd632fa06cd in syn::buffer::TokenBuffer::new2 (stream=...) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/buffer.rs:112
#26 0x00007fd632e966a3 in syn::parse::{{impl}}::parse2<fn(&syn::parse::ParseBuffer) -> core::result::Result<syn::derive::DeriveInput, syn::error::Error>,syn::derive::DeriveInput> (self=0x4, tokens=...) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/parse.rs:1207
#27 0x00007fd632e9656a in syn::parse::Parser::parse<fn(&syn::parse::ParseBuffer) -> core::result::Result<syn::derive::DeriveInput, syn::error::Error>> (self=0x7fd632ed5450 <proc_macro::bridge::client::{{impl}}::expand1::run<fn(proc_macro::TokenStream) -> proc_macro::TokenStream>>, tokens=...) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/parse.rs:1161
#28 0x00007fd632e96278 in syn::parse_macro_input::parse<syn::derive::DeriveInput> (token_stream=...) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.69/src/parse_macro_input.rs:139
#29 0x00007fd632eaadb2 in serde_derive::derive_deserialize (input=...) at /home/marco/.cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/lib.rs:93
#30 0x00007fd632e97d05 in core::ops::function::FnOnce::call_once<fn(proc_macro::TokenStream) -> proc_macro::TokenStream,(proc_macro::TokenStream)> () at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
#31 0x00007fd632ed54f6 in proc_macro::bridge::client::{{impl}}::expand1::run::{{closure}}<fn(proc_macro::TokenStream) -> proc_macro::TokenStream> (input=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:410
#32 0x00007fd632ed4b4f in proc_macro::bridge::client::run_client::{{closure}}::{{closure}}<proc_macro::bridge::client::TokenStream,proc_macro::bridge::client::TokenStream,closure-0> () at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:377
#33 0x00007fd632e95188 in proc_macro::bridge::scoped_cell::{{impl}}::set::{{closure}}<proc_macro::bridge::client::BridgeStateL,(),closure-0> () at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:80
#34 0x00007fd632e9539d in proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>::replace<proc_macro::bridge::client::BridgeStateL,(),closure-0> (self=0x7fd63c1fdb28, replacement=..., f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:75
#35 0x00007fd632e95161 in proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>::set<proc_macro::bridge::client::BridgeStateL,(),closure-0> (self=0x7fd63c1fdb28, value=..., f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:80
#36 0x00007fd632ed52bf in proc_macro::bridge::client::{{impl}}::enter::{{closure}}<(),closure-0> (state=0x7fd63c1fdb28) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:325
#37 0x00007fd632e863ed in std::thread::local::LocalKey<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>::try_with<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>,closure-1,()> (self=0x7fd633222f00, f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:272
#38 0x00007fd632e85eed in std::thread::local::LocalKey<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>>::with<proc_macro::bridge::scoped_cell::ScopedCell<proc_macro::bridge::client::BridgeStateL>,closure-1,()> (self=0x7fd633222f00, f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:248
#39 0x00007fd632e855dd in proc_macro::bridge::Bridge::enter<(),closure-0> (self=..., f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:325
#40 0x00007fd632ed4a44 in proc_macro::bridge::client::run_client::{{closure}}<proc_macro::bridge::client::TokenStream,proc_macro::bridge::client::TokenStream,closure-0> () at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:370
#41 0x00007fd632e92f62 in std::panic::{{impl}}::call_once<(),closure-0> (self=..., _args=()) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:344
#42 0x00007fd632ed6330 in std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure-0>,()> (data=0x7fd63c1eb808) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:379
#43 0x00007fd632ee007d in __rust_try () from /home/marco/Progetti/temp/target/debug/deps/libserde_derive-0445a282a2c7250e.so
#44 0x00007fd632ed6250 in std::panicking::try<(),std::panic::AssertUnwindSafe<closure-0>> (f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:343
#45 0x00007fd632e96232 in std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure-0>,()> (f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:431
#46 0x00007fd632ed489c in proc_macro::bridge::client::run_client<proc_macro::bridge::client::TokenStream,proc_macro::bridge::client::TokenStream,closure-0> (bridge=..., f=...) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:369
#47 0x00007fd632ed54b1 in proc_macro::bridge::client::{{impl}}::expand1::run<fn(proc_macro::TokenStream) -> proc_macro::TokenStream> (bridge=..., f=0x7fd632eaad80 <serde_derive::derive_deserialize>) at /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:410
#48 0x00007fd644e0bd50 in proc_macro::bridge::server::run_server () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#49 0x00007fd644ddb840 in <rustc_expand::proc_macro::ProcMacroDerive as rustc_expand::base::MultiItemModifier>::expand () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#50 0x00007fd64423ae0b in rustc_expand::expand::MacroExpander::fully_expand_fragment () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#51 0x00007fd644dd5e50 in rustc_expand::expand::MacroExpander::expand_crate () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#52 0x00007fd64461f7da in rustc_session::utils::<impl rustc_session::session::Session>::time () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#53 0x00007fd644633932 in rustc_interface::passes::configure_and_expand_inner () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#54 0x00007fd644629eb5 in rustc_interface::passes::configure_and_expand::_$u7b$$u7b$closure$u7d$$u7d$::h3a41a297ff107384 () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#55 0x00007fd64462344d in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#56 0x00007fd644632e4b in rustc_interface::passes::configure_and_expand () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#57 0x00007fd6446477ea in rustc_interface::queries::Queries::expansion () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#58 0x00007fd6445dcba8 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#59 0x00007fd6445d6643 in rustc_span::with_source_map () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#60 0x00007fd6445ddeca in rustc_interface::interface::create_compiler_and_run () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#61 0x00007fd6445d6d05 in rustc_span::with_session_globals () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#62 0x00007fd6445de36a in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#63 0x00007fd6445fac4a in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/marco/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-7ea116e55de24565.so
#64 0x00007fd641a47c8a in alloc::boxed::{{impl}}::call_once<(),FnOnce<()>,alloc::alloc::Global> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/alloc/src/boxed.rs:1521
#65 alloc::boxed::{{impl}}::call_once<(),alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>,alloc::alloc::Global> () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0/library/alloc/src/boxed.rs:1521
#66 std::sys::unix::thread::{{impl}}::new::thread_start () at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0//library/std/src/sys/unix/thread.rs:71
#67 0x00007fd64196d450 in start_thread (arg=0x7fd63c1ff640) at pthread_create.c:473
#68 0x00007fd64187bd53 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
