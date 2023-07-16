
#0  0x000000006399edb4 in syn::path::parsing::<impl syn::path::Path>::get_ident () from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#1  0x00000000638ceef1 in core::iter::traits::iterator::Iterator::try_for_each::call::{{closure}} ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#2  0x00000000638c1582 in <core::iter::adapters::FilterMap<I,F> as core::iter::traits::iterator::Iterator>::next ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#3  0x00000000638eddf8 in serde_derive::internals::attr::Variant::from_ast ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#4  0x00000000638dbc20 in <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::next ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#5  0x00000000638e0c38 in serde_derive::internals::ast::Container::from_ast ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#6  0x0000000063907224 in serde_derive::de::expand_derive_deserialize ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#7  0x0000000063964e72 in serde_derive::derive_deserialize ()
   from \\?\F:\zen_test\target\release\deps\serde_derive-e49d39054da7e2ae.dll
#8  0x00000000639caa44 in proc_macro::bridge::client::__run_expand1::{{closure}}::{{closure}} () at src\libproc_macro\bridge/client.rs:358
#9  proc_macro::bridge::scoped_cell::ScopedCell<T>::set::{{closure}} ()
    at src\libproc_macro\bridge/scoped_cell.rs:79
#10 proc_macro::bridge::scoped_cell::ScopedCell<T>::replace ()
    at src\libproc_macro\bridge/scoped_cell.rs:74
#11 proc_macro::bridge::scoped_cell::ScopedCell<T>::set ()
    at src\libproc_macro\bridge/scoped_cell.rs:79
#12 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}} () at src\libproc_macro\bridge/client.rs:309
#13 std::thread::local::LocalKey<T>::try_with ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\libstd\thread/local.rs:262
#14 std::thread::local::LocalKey<T>::with ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\libstd\thread/local.rs:239
#15 proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter ()
    at src\libproc_macro\bridge/client.rs:309
#16 proc_macro::bridge::client::__run_expand1::{{closure}} ()
    at src\libproc_macro\bridge/client.rs:351
#17 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\libstd/panic.rs:315
#18 std::panicking::try::do_call ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\libstd/panicking.rs:296
#19 0x0000000063a28019 in __rust_maybe_catch_panic ()
    at src\libpanic_unwind\lib.rs:80
#20 0x00000000639d100e in std::panicking::try ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\libstd/panicking.rs:275
#21 std::panic::catch_unwind ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\libstd/panic.rs:394
#22 proc_macro::bridge::client::__run_expand1 ()
    at src\libproc_macro\bridge/client.rs:350
#23 0x0000000002ad0c6d in proc_macro::bridge::server::run_server ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#24 0x0000000002bba233 in <syntax::ext::proc_macro::ProcMacroDerive as syntax::ext::base::MultiItemModifier>::expand ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#25 0x0000000002bafa58 in syntax::ext::expand::MacroExpander::fully_expand_fragment ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#26 0x0000000002baeabd in syntax::ext::expand::MacroExpander::expand_crate ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#27 0x0000000000ff6312 in rustc_interface::passes::configure_and_expand_inner::{{closure}} ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#28 0x0000000000fe89a7 in rustc::util::common::time ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#29 0x0000000000f6859d in rustc_interface::passes::configure_and_expand_inner
    ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#30 0x0000000000fcb48d in rustc_interface::passes::configure_and_expand::{{closure}} ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#31 0x0000000000f9efff in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#32 0x0000000000f6ee01 in rustc_interface::queries::Query<T>::compute ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#33 0x0000000000ff73da in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::expansion ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#34 0x0000000000e75a68 in rustc_interface::interface::run_compiler_in_existing_thread_pool ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#35 0x0000000000e9c8af in std::thread::local::LocalKey<T>::with ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#36 0x0000000000eb0eda in scoped_tls::ScopedKey<T>::set ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#37 0x0000000000ecf781 in syntax::with_globals ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#38 0x0000000000e4fb2d in std::sys_common::backtrace::__rust_begin_short_backtrace ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#39 0x0000000066c75859 in __rust_maybe_catch_panic ()
    at src\libpanic_unwind\lib.rs:80
#40 0x0000000000e781b3 in core::ops::function::FnOnce::call_once{{vtable-shim}} ()
   from C:\Users\mateusz\.rustup\toolchains\nightly-x86_64-pc-windows-gnu\bin\rustc_driver-dc847589f2996fc8.dll
#41 0x0000000066c46836 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\liballoc/boxed.rs:922
#42 0x0000000066c72dd7 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once ()
    at /rustc/17e73e801a75559eac5c932ff07bd9c8499a1364\src\liballoc/boxed.rs:922
#43 std::sys_common::thread::start_thread ()
    at src\libstd\sys_common/thread.rs:13
#44 std::sys::windows::thread::Thread::new::thread_start ()
    at src\libstd\sys\windows/thread.rs:47
#45 0x00007ffcaec77bd4 in KERNEL32!BaseThreadInitThunk ()
   from C:\WINDOWS\System32\kernel32.dll
#46 0x00007ffcaedcce71 in ntdll!RtlUserThreadStart ()
   from C:\WINDOWS\SYSTEM32\ntdll.dll
#47 0x0000000000000000 in ?? ()
Backtrace stopped: previous frame inner to this frame (corrupt stack?)
