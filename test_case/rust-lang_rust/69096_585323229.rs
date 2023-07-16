rust
(gdb) bt
#0  0x00007ffff6966e15 in hashbrown::map::RawEntryBuilder<K,V,S>::from_key_hashed_nocheck () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#1  0x00007ffff6d36f5e in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#2  0x00007ffff6a6a0c3 in rustc::ty::structural_impls::fold_list () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#3  0x00007ffff6d00036 in rustc::ty::normalize_erasing_regions::<impl rustc::ty::context::TyCtxt>::normalize_erasing_late_bound_regions () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#4  0x00007ffff6fd048a in rustc::ty::print::obsolete::DefPathBasedNames::push_type_name () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#5  0x00007ffff6fd2d39 in rustc::ty::print::obsolete::DefPathBasedNames::push_generic_params () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#6  0x00007ffff6fd1739 in rustc::ty::print::obsolete::DefPathBasedNames::push_type_name () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#7  0x00007ffff6fd2d39 in rustc::ty::print::obsolete::DefPathBasedNames::push_generic_params () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#8  0x00007ffff6fd1739 in rustc::ty::print::obsolete::DefPathBasedNames::push_type_name () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
#9  0x00007ffff6fd2d39 in rustc::ty::print::obsolete::DefPathBasedNames::push_generic_params () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-154376714aaee7fe.so
...
#182017 0x00007ffff6eddcce in rustc::ty::print::obsolete::DefPathBasedNames::push_type_name () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182018 0x00007ffff6eded19 in rustc::ty::print::obsolete::DefPathBasedNames::push_generic_params () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182019 0x00007ffff50016fc in <rustc_target::abi::TyLayout<&rustc::ty::TyS> as rustc_codegen_llvm::type_of::LayoutLlvmExt>::llvm_type () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182020 0x00007ffff4eb5188 in rustc_codegen_ssa::mir::operand::OperandRef<V>::new_zst () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182021 0x00007ffff4fbd16d in rustc_codegen_ssa::mir::codegen_mir::{{closure}} () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182022 0x00007ffff4fe5216 in <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::fold () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182023 0x00007ffff4fb23f2 in <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182024 0x00007ffff4fbc205 in rustc_codegen_ssa::mir::codegen_mir () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182025 0x00007ffff4ef2bfc in <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182026 0x00007ffff4e6859a in rustc_codegen_llvm::base::compile_codegen_unit::module_codegen () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182027 0x00007ffff4f1c8b0 in rustc::dep_graph::graph::DepGraph::with_task () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182028 0x00007ffff4e68164 in rustc_codegen_llvm::base::compile_codegen_unit () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182029 0x00007ffff4fec61f in rustc_codegen_ssa::base::codegen_crate () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182030 0x00007ffff4f297b5 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182031 0x00007ffff4deeccc in rustc_interface::passes::QueryContext::enter () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182032 0x00007ffff4d120bb in rustc_interface::queries::Queries::ongoing_codegen () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182033 0x00007ffff4bae7d3 in rustc_interface::interface::run_compiler_in_existing_thread_pool () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182034 0x00007ffff4b664bd in scoped_tls::ScopedKey<T>::set () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182035 0x00007ffff4bd3414 in syntax::attr::with_globals () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182036 0x00007ffff4b9df10 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182037 0x00007ffff44ece67 in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:86
#182038 0x00007ffff4bb0c36 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/lzutao/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-d8719fb60c037747.so
#182039 0x00007ffff44b846f in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/a1912f2e89b77cfe2a0e64b96f444848fe4e2d49/src/liballoc/boxed.rs:1017
#182040 0x00007ffff44eb770 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/a1912f2e89b77cfe2a0e64b96f444848fe4e2d49/src/liballoc/boxed.rs:1017
#182041 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#182042 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:80
#182043 0x00007ffff4433fa3 in start_thread (arg=<optimized out>) at pthread_create.c:486
#182044 0x00007ffff43534cf in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
(gdb)
