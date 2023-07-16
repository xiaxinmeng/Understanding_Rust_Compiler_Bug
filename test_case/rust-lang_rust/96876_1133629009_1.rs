
$ gdb --args rustc test1.rs
(gdb) run
Starting program: /usr/bin/rustc test1.rs
[New LWP 1800]

Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to LWP 1800]
0x0000003ff74dcff4 in <rustc_span::span_encoding::Span as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
(gdb) bt
#0  0x0000003ff74dcff4 in <rustc_span::span_encoding::Span as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable ()
   from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#1  0x0000003ff74cd6a6 in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_hir_expr::{closure#0} () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#2  0x0000003ff74cdbf4 in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_hir_expr::{closure#0} () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#3  0x0000003ff74cf184 in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_hir_expr::{closure#0} () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#4  0x0000003ff74cdadc in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_hir_expr::{closure#0} () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#5  0x0000003ff74e039a in <&[rustc_hir::hir::Stmt] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#6  0x0000003ff74cdefc in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_hir_expr::{closure#0} () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#7  0x0000003ff74cd60c in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_hir_expr () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#8  0x0000003ff63e1e56 in <rustc_query_system::ich::hcx::StableHashingContext as rustc_hir::stable_hash_impls::HashStableContext>::hash_body_id () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#9  0x0000003ff64241d0 in <rustc_hir::hir::OwnerNode as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#10 0x0000003ff63f4590 in <rustc_ast_lowering::LoweringContext>::make_owner_info () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#11 0x0000003ff63dc138 in <rustc_ast_lowering::item::ItemLowerer as rustc_ast::visit::Visitor>::visit_item () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#12 0x0000003ff63f0340 in rustc_ast_lowering::lower_crate () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#13 0x0000003ff5d86122 in <rustc_interface::queries::Queries>::global_ctxt () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#14 0x0000003ff5c94808 in rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0} ()
   from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#15 0x0000003ff5cc6ecc in std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>> ()
   from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#16 0x0000003ff5c8ff4a in <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} () from /usr/bin/../lib/librustc_driver-e81ecd0b9419698e.so
#17 0x0000003ff558bde6 in std::sys::unix::thread::Thread::new::thread_start () from /usr/bin/../lib/libstd-5a982fb5ee18f2d1.so
#18 0x0000003ff7fbbb72 in start (p=0x3fefb706b8) at src/thread/pthread_create.c:203
#19 0x0000003ff7fbd248 in __clone () at src/thread/riscv64/clone.s:30
Backtrace stopped: frame did not save the PC
