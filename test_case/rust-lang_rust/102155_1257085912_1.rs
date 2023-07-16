
Reading symbols from /home/luna/.cargo/bin/rustc...
(gdb) set args --crate-name foo --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1b3b0ae194b7a453 -C extra-filename=-1b3b0ae194b7a453 --out-dir /home/luna/foo/target/debug/deps -C incremental=/home/luna/foo/target/debug/incremental -L dependency=/home/luna/foo/target/debug/deps
(gdb) run
Starting program: /home/luna/.cargo/bin/rustc --crate-name foo --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1b3b0ae194b7a453 -C extra-filename=-1b3b0ae194b7a453 --out-dir /home/luna/foo/target/debug/deps -C incremental=/home/luna/foo/target/debug/incremental -L dependency=/home/luna/foo/target/debug/deps
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/riscv64-linux-gnu/libthread_db.so.1".
process 28658 is executing new program: /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/rustc
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/riscv64-linux-gnu/libthread_db.so.1".
[New Thread 0x3ff05f1ca0 (LWP 28660)]
[New Thread 0x3fefdedca0 (LWP 28661)]
[Thread 0x3fefdedca0 (LWP 28661) exited]
{"artifact":"/home/luna/foo/target/debug/deps/foo-1b3b0ae194b7a453.d","emit":"dep-info"}

Thread 2 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x3ff05f1ca0 (LWP 28660)]
0x0000003ff45f21aa in <rustc_middle::arena::Arena>::alloc_from_iter::<rustc_middle::dep_graph::dep_node::DepKindStruct, rustc_arena::IsNotCopy, [rustc_middle::dep_graph::dep_node::DepKindStruct; 282]> () from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
(gdb) bt full
#0  0x0000003ff45f21aa in <rustc_middle::arena::Arena>::alloc_from_iter::<rustc_middle::dep_graph::dep_node::DepKindStruct, rustc_arena::IsNotCopy, [rustc_middle::dep_graph::dep_node::DepKindStruct; 282]> () from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#1  0x0000003ff47f8baa in rustc_query_impl::query_callbacks () from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#2  0x0000003ff124f00a in <core::cell::once::OnceCell<_>>::get_or_try_init::outlined_call::<<core::cell::once::OnceCell<rustc_middle::ty::context::GlobalCtxt>>::get_or_init<rustc_interface::passes::create_global_ctxt::{closure#1}::{closure#0}>::{closure#0}, rustc_middle::ty::context::GlobalCtxt, !> ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#3  0x0000003ff165fd14 in <core::cell::once::OnceCell<rustc_middle::ty::context::GlobalCtxt>>::get_or_init::<rustc_interface::passes::create_global_ctxt::{closure#1}::{closure#0}>
    () from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#4  0x0000003ff15d6018 in <rustc_session::session::Session>::time::<&rustc_middle::ty::context::GlobalCtxt, rustc_interface::passes::create_global_ctxt::{closure#1}> ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#5  0x0000003ff15d4b48 in rustc_interface::passes::create_global_ctxt ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#6  0x0000003ff164e794 in <rustc_interface::queries::Queries>::global_ctxt ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#7  0x0000003ff14f4ada in <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>> () from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#8  0x0000003ff15407a6 in rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}> ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#9  0x0000003ff14f58d4 in <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>> ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#10 0x0000003ff1512996 in std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>> () from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#11 0x0000003ff150c666 in _RINvNvNtCseOBki07ryB6_3std9panicking3try7do_callINtNtNtCsidPuqEqzKzv_4core5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB6_6threadNtB1T_7Builder16spawn_unchecked_NCINvNtCs2PGdSkTarcu_15rustc_interface4util31run_in_thread_pool_with_globalsNCINvNtB2I_9interface12run_compilerINtNtBR_6result6ResultuNtCs4NYEZz9yNmi_12rustc_errors15ErrorGuaranteedENCNvCs2vGVMgUuDv2_12rustc_driver12run_compilers_0E0B4o_E0B4o_Es_00EB4o_EB5B_.llvm.13548527062024321570 ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#12 0x0000003ff150eda4 in __rust_try.llvm.13548527062024321570 ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#13 0x0000003ff15136da in <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} ()
   from /home/luna/.rustup/toolchains/stable-riscv64gc-unknown-linux-gnu/bin/../lib/librustc_driver-ac972a4e10c98556.so
No symbol table info available.
#14 0x0000003ff085a542 in alloc::boxed::{impl#44}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1935
No locals.
--Type <RET> for more, q to quit, c to continue without paging--c
#15 alloc::boxed::{impl#44}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1935
No locals.
#16 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
No locals.
#17 0x0000003ff06ea5a6 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
        ret = <optimized out>
        start = <optimized out>
        pd = <optimized out>
        out = <optimized out>
        unwind_buf = {cancel_jmp_buf = {{jmp_buf = {{__pc = 274616722770, __regs = {274615702736, 274615704736, 274877885078, 274877885079, 274743680272, 0, 274618230044, 6, 274743680272, 274607316992, 274615704736, 274615704736}, __sp = 274615702416, __fpregs = {0 <repeats 12 times>}}}, mask_was_saved = 0}}, priv = {pad = {0x0, 0x0, 0x0, 0x0}, data = {prev = 0x0, cleanup = 0x0, canceltype = 0}}}
        not_first_call = <optimized out>
        robust = <optimized out>
#18 0x0000003ff0736a02 in __thread_start () at ../sysdeps/unix/sysv/linux/riscv/clone.S:85
No locals.
(gdb)
