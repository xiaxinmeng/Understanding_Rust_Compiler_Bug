
#0  0x00007ffff305133a in raise () from /nix/store/m0xa5bz7vw7p43wi0jppvvi3c9vgqvp7-glibc-2.32-25/lib/libc.so.6
#1  0x00007ffff303b523 in abort () from /nix/store/m0xa5bz7vw7p43wi0jppvvi3c9vgqvp7-glibc-2.32-25/lib/libc.so.6
#2  0x00007ffff32b65ba in std::sys::unix::abort_internal () at /rustc/23adf9fd843da7a3394c824b056f93151aaa40ad//library/std/src/sys/unix/mod.rs:237
#3  0x00007ffff329e9b9 in std::process::abort () at /rustc/23adf9fd843da7a3394c824b056f93151aaa40ad//library/std/src/process.rs:1784
#4  0x00007ffff5b0c084 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::visit_expr () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#5  0x00007ffff5b29779 in rustc_ast::mut_visit::noop_visit_expr () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#6  0x00007ffff5b45eba in rustc_ast::ptr::P<T>::filter_map () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#7  0x00007ffff5b2ede1 in rustc_ast::mut_visit::noop_flat_map_stmt_kind () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#8  0x00007ffff5b0d184 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_stmt () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#9  0x00007ffff5afe2bf in <alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#10 0x00007ffff5b2d809 in rustc_ast::mut_visit::noop_visit_item_kind () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#11 0x00007ffff5b2c101 in rustc_ast::mut_visit::noop_flat_map_item () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#12 0x00007ffff5b0df91 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#13 0x00007ffff5afb907 in <alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#14 0x00007ffff5b2d9c4 in rustc_ast::mut_visit::noop_visit_item_kind () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#15 0x00007ffff5b2c101 in rustc_ast::mut_visit::noop_flat_map_item () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#16 0x00007ffff5b0df91 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#17 0x00007ffff66d69e7 in <smallvec::SmallVec<A> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#18 0x00007ffff5b0a515 in rustc_expand::expand::MacroExpander::collect_invocations () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#19 0x00007ffff5b05efc in rustc_expand::expand::MacroExpander::fully_expand_fragment () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#20 0x00007ffff66aa780 in rustc_expand::expand::MacroExpander::expand_crate () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#21 0x00007ffff5eeec1a in rustc_session::utils::<impl rustc_session::session::Session>::time () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#22 0x00007ffff5f00d1b in rustc_interface::passes::configure_and_expand_inner () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#23 0x00007ffff5ef7275 in rustc_interface::passes::configure_and_expand::_$u7b$$u7b$closure$u7d$$u7d$::h5397698db2b97e0d () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#24 0x00007ffff5ef0756 in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#25 0x00007ffff5f00234 in rustc_interface::passes::configure_and_expand () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#26 0x00007ffff5f14a8a in rustc_interface::queries::Queries::expansion () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#27 0x00007ffff5ea9cac in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#28 0x00007ffff5ea3663 in rustc_span::with_source_map () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#29 0x00007ffff5eaafca in rustc_interface::interface::create_compiler_and_run () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#30 0x00007ffff5ea3d2e in rustc_span::with_session_globals () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#31 0x00007ffff5eab473 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#32 0x00007ffff5ec7e2a in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /nix/store/hkv7bckn0vryzcf9x33cv4x3jqd1s513-rustc/lib/librustc_driver-197bed005be091ad.so
#33 0x00007ffff32b5dba in alloc::boxed::{{impl}}::call_once<(),FnOnce<()>,alloc::alloc::Global> () at /rustc/23adf9fd843da7a3394c824b056f93151aaa40ad/library/alloc/src/boxed.rs:1521
#34 alloc::boxed::{{impl}}::call_once<(),alloc::boxed::Box<FnOnce<()>, alloc::alloc::Global>,alloc::alloc::Global> () at /rustc/23adf9fd843da7a3394c824b056f93151aaa40ad/library/alloc/src/boxed.rs:1521
#35 std::sys::unix::thread::{{impl}}::new::thread_start () at /rustc/23adf9fd843da7a3394c824b056f93151aaa40ad//library/std/src/sys/unix/thread.rs:71
#36 0x00007ffff31eee9e in start_thread () from /nix/store/m0xa5bz7vw7p43wi0jppvvi3c9vgqvp7-glibc-2.32-25/lib/libpthread.so.0
#37 0x00007ffff311058f in clone () from /nix/store/m0xa5bz7vw7p43wi0jppvvi3c9vgqvp7-glibc-2.32-25/lib/libc.so.6
