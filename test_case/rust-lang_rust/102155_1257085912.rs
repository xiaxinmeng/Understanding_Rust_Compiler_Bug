
(gdb) bt full
#0  syscall (syscall_number=98, arg1=<optimized out>, arg2=137, arg3=0, arg4=0, arg5=0, arg6=-1, arg7=274609475632) at ../sysdeps/unix/sysv/linux/riscv/syscall.c:27
        ret = <optimized out>
#1  0x0000002aab379664 in std::sys::unix::futex::futex_wait () at library/std/src/sys/unix/futex.rs:62
No locals.
#2  0x0000002aab37c50e in std::sys::unix::locks::futex_condvar::Condvar::wait_optional_timeout () at library/std/src/sys/unix/locks/futex_condvar.rs:51
No locals.
#3  std::sys::unix::locks::futex_condvar::Condvar::wait () at library/std/src/sys/unix/locks/futex_condvar.rs:35
No locals.
#4  0x0000002aab3504d4 in <jobserver::HelperState>::for_each_request::<jobserver::imp::spawn_helper::{closure#1}::{closure#0}> ()
No symbol table info available.
#5  0x0000002aab350a60 in std::sys_common::backtrace::__rust_begin_short_backtrace::<jobserver::imp::spawn_helper::{closure#1}, ()> ()
No symbol table info available.
#6  0x0000002aab350cb2 in _RINvNvNtCseOBki07ryB6_3std9panicking3try7do_callINtNtNtCsidPuqEqzKzv_4core5panic11unwind_safe16AssertUnwindSafeNCNCINvMNtB6_6threadNtB1T_7Builder16spawn_unchecked_NCNvNtCsGjmX1GWYch_9jobserver3imp12spawn_helpers_0uEs_00EuEB2H_.llvm.3138756864971081497 ()
No symbol table info available.
#7  0x0000002aab350d4e in __rust_try.llvm.3138756864971081497 ()
No symbol table info available.
#8  0x0000002aab351954 in <<std::thread::Builder>::spawn_unchecked_<jobserver::imp::spawn_helper::{closure#1}, ()>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} ()
No symbol table info available.
#9  0x0000002aab37bdc0 in alloc::boxed::{impl#44}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> () at library/alloc/src/boxed.rs:1935
No locals.
#10 alloc::boxed::{impl#44}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> ()
    at library/alloc/src/boxed.rs:1935
No locals.
#11 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
No locals.
#12 0x0000003ff7e7c5a6 in start_thread (arg=<optimized out>) at ./nptl/pthread_create.c:442
        ret = <optimized out>
        start = <optimized out>
        pd = <optimized out>
        out = <optimized out>
        unwind_buf = {cancel_jmp_buf = {{jmp_buf = {{__pc = 274742101330, __regs = {274741663600, 274741665600, 274877883878, 274877883879, 274743680272, 0, 183261183386, 6,
                    274743680272, 274739568640, 274741665600, 274741665600}, __sp = 274741663280, __fpregs = {0 <repeats 12 times>}}}, mask_was_saved = 0}}, priv = {pad = {0x0,
              0x0, 0x0, 0x0}, data = {prev = 0x0, cleanup = 0x0, canceltype = 0}}}
        not_first_call = <optimized out>
        robust = <optimized out>
#13 0x0000003ff7ec8a02 in __thread_start () at ../sysdeps/unix/sysv/linux/riscv/clone.S:85
No locals.
(gdb)
