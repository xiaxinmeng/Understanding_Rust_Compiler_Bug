
Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x3ff05efd20 (LWP 2816)]
0x0000003ff45f61aa in <rustc_middle::arena::Arena>::alloc_from_iter::<rustc_middle::dep_graph::dep_node::DepKindStruct, rustc_arena::IsNotCopy, [rustc_middle::dep_graph::dep_node::DepKindStruct; 282]> () from /home/tommy/.rustup.riscv64-linux/toolchains/1.64.0-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
(gdb) bt
#0  0x0000003ff45f61aa in <rustc_middle::arena::Arena>::alloc_from_iter::<rustc_middle::dep_graph::dep_node::DepKindStruct, rustc_arena::IsNotCopy, [rustc_middle::dep_graph::dep_node::DepKindStruct; 282]> ()
   from /home/tommy/.rustup.riscv64-linux/toolchains/1.64.0-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
#1  0x0000003ff47fcbaa in rustc_query_impl::query_callbacks () from /home/tommy/.rustup.riscv64-linux/toolchains/1.64.0-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
#2  0x0000003ff125300a in <core::cell::once::OnceCell<_>>::get_or_try_init::outlined_call::<<core::cell::once::OnceCell<rustc_middle::ty::context::GlobalCtxt>>::get_or_init<rustc_interface::passes::create_global_ctxt::{closure#1}::{closure#0}>::{closure#0}, rustc_middle::ty::context::GlobalCtxt, !> ()
   from /home/tommy/.rustup.riscv64-linux/toolchains/1.64.0-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
#3  0x0000003ff1663d14 in <core::cell::once::OnceCell<rustc_middle::ty::context::GlobalCtxt>>::get_or_init::<rustc_interface::passes::create_global_ctxt::{closure#1}::{closure#0}> ()
...
