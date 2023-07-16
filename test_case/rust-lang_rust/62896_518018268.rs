
gdb --args rustc --crate-name build_script_build /home/fenhl/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.60/build.rs --color always --crate-type bin --emit=dep-info,link -C opt-level=3 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=60aaac3506c3c4f4 -C extra-filename=-60aaac3506c3c4f4 --out-dir /home/fenhl/compile-fail/target/release/build/libc-60aaac3506c3c4f4 -L dependency=/home/fenhl/compile-fail/target/release/deps --cap-lints allow
GNU gdb (Raspbian 8.2.1-2) 8.2.1
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "arm-linux-gnueabihf".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from rustc...done.
(gdb) r
Starting program: /home/fenhl/.cargo/bin/rustc --crate-name build_script_build /home/fenhl/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.60/build.rs --color always --crate-type bin --emit=dep-info,link -C opt-level=3 --cfg feature=\"default\" --cfg feature=\"std\" -C metadata=60aaac3506c3c4f4 -C extra-filename=-60aaac3506c3c4f4 --out-dir /home/fenhl/compile-fail/target/release/build/libc-60aaac3506c3c4f4 -L dependency=/home/fenhl/compile-fail/target/release/deps --cap-lints allow
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
process 22509 is executing new program: /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/rustc
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
[New Thread 0x74b471e0 (LWP 22524)]
[New Thread 0x6d7fe1e0 (LWP 22549)]
[New Thread 0x6d3fe1e0 (LWP 22550)]
[New Thread 0x6cffe1e0 (LWP 22551)]
[Thread 0x6cffe1e0 (LWP 22551) exited]
[New Thread 0x6cffe1e0 (LWP 22554)]
[New Thread 0x6c9fe1e0 (LWP 22555)]
[New Thread 0x6c7fd1e0 (LWP 22556)]
[New Thread 0x6c3fe1e0 (LWP 22557)]
[New Thread 0x6bffe1e0 (LWP 22558)]
[New Thread 0x6bdfd1e0 (LWP 22559)]
[New Thread 0x6bbfc1e0 (LWP 22560)]
[New Thread 0x6b9fb1e0 (LWP 22561)]
[Thread 0x6bffe1e0 (LWP 22558) exited]
[Thread 0x6c9fe1e0 (LWP 22555) exited]
[New Thread 0x6bffe1e0 (LWP 22562)]
[Thread 0x6bdfd1e0 (LWP 22559) exited]
[Thread 0x6c7fd1e0 (LWP 22556) exited]
[Thread 0x6b9fb1e0 (LWP 22561) exited]
[New Thread 0x6b9fb1e0 (LWP 22563)]
[Thread 0x6bffe1e0 (LWP 22562) exited]
[Thread 0x6bbfc1e0 (LWP 22560) exited]
[New Thread 0x6bbfc1e0 (LWP 22564)]
[Thread 0x6bbfc1e0 (LWP 22564) exited]
[Thread 0x6b9fb1e0 (LWP 22563) exited]
[New Thread 0x6b9fb1e0 (LWP 22565)]
[New Thread 0x6bbfc1e0 (LWP 22566)]
[New Thread 0x6bffe1e0 (LWP 22567)]
[Thread 0x6b9fb1e0 (LWP 22565) exited]
[Thread 0x6bbfc1e0 (LWP 22566) exited]
[Thread 0x6bffe1e0 (LWP 22567) exited]
[Thread 0x6cffe1e0 (LWP 22554) exited]
[Thread 0x6c3fe1e0 (LWP 22557) exited]
[New Thread 0x6c3fe1e0 (LWP 22568)]
[New Thread 0x6cffe1e0 (LWP 22569)]
[New Thread 0x6bffe1e0 (LWP 22570)]
[New Thread 0x6bbfc1e0 (LWP 22571)]
[New Thread 0x6b9fb1e0 (LWP 22572)]
[New Thread 0x6c7fd1e0 (LWP 22573)]
[New Thread 0x6bdfd1e0 (LWP 22574)]
[New Thread 0x6c9fe1e0 (LWP 22575)]
[New Thread 0x6b7fa1e0 (LWP 22576)]
[New Thread 0x6b5f91e0 (LWP 22577)]
[New Thread 0x6b3f81e0 (LWP 22578)]
malloc(): invalid next size (unsorted)
[New Thread 0x6b1f71e0 (LWP 22579)]
rustc: malloc.c:4028: _int_malloc: Assertion `(unsigned long) (size) >= (unsigned long) (nb)' failed.
[New Thread 0x6aff61e0 (LWP 22580)]

Thread 29 "rustc" received signal SIGSEGV, Segmentation fault.
[Switching to Thread 0x6b5f91e0 (LWP 22577)]
0x74d1ab7c in _int_malloc (av=av@entry=0x6c400010, bytes=bytes@entry=1616) at malloc.c:4033
4033	malloc.c: No such file or directory.
(gdb) set pagination off
(gdb) thread apply all bt

Thread 32 (Thread 0x6aff61e0 (LWP 22580)):
#0  clone () at ../sysdeps/unix/sysv/linux/arm/clone.S:58
#1  0x74c1d180 in create_thread (pd=0x6aff6248, attr=0x6d3fced8, stopped_start=0x6d3fce82, stackaddr=<optimized out>, thread_ran=0x0) at ../sysdeps/unix/sysv/linux/createthread.c:101
#2  0x00000000 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 31 (Thread 0x6b1f71e0 (LWP 22579)):
#0  0x70b45cb6 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x6d04dcf8 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 30 (Thread 0x6b3f81e0 (LWP 22578)):
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x74cc0230 in __GI_abort () at abort.c:79
#2  0x74d1051c in __libc_message (action=action@entry=do_abort, fmt=<optimized out>) at ../sysdeps/posix/libc_fatal.c:181
#3  0x74d17044 in malloc_printerr (str=<optimized out>) at malloc.c:5341
#4  0x74d1a6d8 in _int_malloc (av=av@entry=0x6cc00010, bytes=bytes@entry=320) at malloc.c:3735
#5  0x74d1b2fc in tcache_init () at malloc.c:2979
#6  0x74d1c1f8 in tcache_init () at malloc.c:3034
#7  __GI___libc_malloc (bytes=32) at malloc.c:3034
#8  0x74c20308 in pthread_getattr_np (thread_id=<optimized out>, attr=0x6b3f7930) at pthread_getattr_np.c:176
#9  0x74ea9298 in std::sys::unix::thread::guard::current () at src/libstd/sys/unix/thread.rs:371
#10 0x7035e3f4 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x74e7fea4 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#12 0x74ea90f0 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#13 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#14 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#15 0x74c1e494 in start_thread (arg=0x6b3f81e0) at pthread_create.c:486
#16 0x74d80578 in ?? () at ../sysdeps/unix/sysv/linux/arm/clone.S:73 from /lib/arm-linux-gnueabihf/libc.so.6
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 29 (Thread 0x6b5f91e0 (LWP 22577)):
#0  0x74d1ab7c in _int_malloc (av=av@entry=0x6c400010, bytes=bytes@entry=1616) at malloc.c:4033
#1  0x74d1c318 in __GI___libc_malloc (bytes=1616) at malloc.c:3057
#2  0x71b5eedc in operator new (sz=1616) at /tmp/build/.build/src/gcc-5.2.0/libstdc++-v3/libsupc++/new_op.cc:50
#3  0x718f3a2c in llvm::LLVMContext::LLVMContext() () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x704b0404 in LLVMRustContextCreate () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#5  0x7039a958 in rustc_codegen_llvm::back::lto::optimize_thin_module () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#6  0x703457b4 in rustc_codegen_ssa::back::lto::LtoModuleCodegen<B>::optimize () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x7040f198 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x7035dab4 in std::panicking::try::do_call () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x74eaa2ec in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:80
#10 0x7035e438 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x74e7fea4 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#12 0x74ea90f0 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#13 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#14 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#15 0x74c1e494 in start_thread (arg=0x6b5f91e0) at pthread_create.c:486
#16 0x74d80578 in ?? () at ../sysdeps/unix/sysv/linux/arm/clone.S:73 from /lib/arm-linux-gnueabihf/libc.so.6
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 28 (Thread 0x6b7fa1e0 (LWP 22576)):
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x74cc0230 in __GI_abort () at abort.c:79
#2  0x74d16fb0 in __malloc_assert (assertion=<optimized out>, file=<optimized out>, line=line@entry=4028, function=0x74dd7514 <__PRETTY_FUNCTION__.17241> "_int_malloc") at malloc.c:298
#3  0x74d1ac6c in _int_malloc (av=av@entry=0x6cb00010, bytes=bytes@entry=252) at malloc.c:4028
#4  0x74d1c318 in __GI___libc_malloc (bytes=252) at malloc.c:3057
#5  0x71b5eedc in operator new (sz=252) at /tmp/build/.build/src/gcc-5.2.0/libstdc++-v3/libsupc++/new_op.cc:50
#6  0x70e0c3e6 in createARMMCAsmInfo(llvm::MCRegisterInfo const&, llvm::Triple const&) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#7  0x711d6c1c in llvm::LLVMTargetMachine::initAsmInfo() () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#8  0x70d9d8e4 in llvm::ARMBaseTargetMachine::ARMBaseTargetMachine(llvm::Target const&, llvm::Triple const&, llvm::StringRef, llvm::StringRef, llvm::TargetOptions const&, llvm::Optional<llvm::Reloc::Model>, llvm::Optional<llvm::CodeModel::Model>, llvm::CodeGenOpt::Level, bool) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#9  0x70d9dac0 in llvm::ARMLETargetMachine::ARMLETargetMachine(llvm::Target const&, llvm::Triple const&, llvm::StringRef, llvm::StringRef, llvm::TargetOptions const&, llvm::Optional<llvm::Reloc::Model>, llvm::Optional<llvm::CodeModel::Model>, llvm::CodeGenOpt::Level, bool) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#10 0x70d9db62 in llvm::RegisterTargetMachine<llvm::ARMLETargetMachine>::Allocator(llvm::Target const&, llvm::Triple const&, llvm::StringRef, llvm::StringRef, llvm::TargetOptions const&, llvm::Optional<llvm::Reloc::Model>, llvm::Optional<llvm::CodeModel::Model>, llvm::CodeGenOpt::Level, bool) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#11 0x704ab25c in LLVMRustCreateTargetMachine () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x70473ab0 in rustc_codegen_llvm::back::write::target_machine_factory::{{closure}} () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x7039a900 in rustc_codegen_llvm::back::lto::optimize_thin_module () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#14 0x703457b4 in rustc_codegen_ssa::back::lto::LtoModuleCodegen<B>::optimize () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#15 0x7040f198 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#16 0x7035dab4 in std::panicking::try::do_call () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#17 0x74eaa2ec in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:80
#18 0x7035e438 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#19 0x74e7fea4 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#20 0x74ea90f0 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#21 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#22 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#23 0x74c1e494 in start_thread (arg=0x6b7fa1e0) at pthread_create.c:486
#24 0x74d80578 in ?? () at ../sysdeps/unix/sysv/linux/arm/clone.S:73 from /lib/arm-linux-gnueabihf/libc.so.6
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 27 (Thread 0x6c9fe1e0 (LWP 22575)):
#0  0x74d1ab7c in _int_malloc (av=av@entry=0x6c000010, bytes=bytes@entry=89) at malloc.c:4033
#1  0x74d1b980 in _int_realloc (av=av@entry=0x6c000010, oldp=oldp@entry=0x6c02eb78, oldsize=oldsize@entry=48, nb=nb@entry=96) at malloc.c:4597
#2  0x74d1cde8 in __GI___libc_realloc (oldmem=0x6c02eb80, bytes=92) at malloc.c:3222
#3  0x71b02216 in llvm::SmallVectorBase::grow_pod(void*, unsigned int, unsigned int) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#4  0x7191a8ec in (anonymous namespace)::GetCFGOnlyPasses::passEnumerate(llvm::PassInfo const*) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 26 (Thread 0x6bdfd1e0 (LWP 22574)):
#0  0x70b45cc8 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x00439338 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 25 (Thread 0x6c7fd1e0 (LWP 22573)):
#0  0x70b45cc4 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x6dc63830 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 24 (Thread 0x6b9fb1e0 (LWP 22572)):
#0  0x70b45c48 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x6d42d000 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 23 (Thread 0x6bbfc1e0 (LWP 22571)):
#0  0x70b45cc0 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
Backtrace stopped: Cannot access memory at address 0x506dad52

Thread 22 (Thread 0x6bffe1e0 (LWP 22570)):
#0  0x70b45cc8 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x6cce2d90 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 21 (Thread 0x6cffe1e0 (LWP 22569)):
#0  0x70b45cc8 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x6c431428 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 20 (Thread 0x6c3fe1e0 (LWP 22568)):
#0  0x70b45cc8 in void std::__insertion_sort<__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first> >(__gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__normal_iterator<std::pair<unsigned long long, llvm::Function*>*, std::vector<std::pair<unsigned long long, llvm::Function*>, std::allocator<std::pair<unsigned long long, llvm::Function*> > > >, __gnu_cxx::__ops::_Iter_comp_iter<llvm::less_first>) () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#1  0x6c043c40 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 4 (Thread 0x6d3fe1e0 (LWP 22550)):
#0  clone () at ../sysdeps/unix/sysv/linux/arm/clone.S:58
#1  0x74c1d180 in create_thread (pd=0x6aff6248, attr=0x6d3fced8, stopped_start=0x6d3fce82, stackaddr=<optimized out>, thread_ran=0x6aff61e0) at ../sysdeps/unix/sysv/linux/createthread.c:101
#2  0x6aff6250 in ?? ()
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 3 (Thread 0x6d7fe1e0 (LWP 22549)):
#0  futex_wait_cancelable (private=0, expected=0, futex_word=0x6dc89ffc) at ../sysdeps/unix/sysv/linux/futex-internal.h:88
#1  __pthread_cond_wait_common (abstime=0x0, mutex=0x0, cond=0x6dc89fd0) at pthread_cond_wait.c:502
#2  __pthread_cond_wait (cond=0x6dc89fd0, mutex=0x0) at pthread_cond_wait.c:655
#3  0x74e80e98 in std::sys::unix::condvar::Condvar::wait () at src/libstd/sys/unix/condvar.rs:69
#4  std::sys_common::condvar::Condvar::wait () at src/libstd/sys_common/condvar.rs:41
#5  std::sync::condvar::Condvar::wait () at src/libstd/sync/condvar.rs:204
#6  std::thread::park () at src/libstd/thread/mod.rs:911
#7  0x74e964f8 in std::sync::mpsc::blocking::WaitToken::wait () at src/libstd/sync/mpsc/blocking.rs:71
#8  0x76c7c0ac in std::sync::mpsc::stream::Packet<T>::recv () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#9  0x76c7fdb4 in std::sync::mpsc::Receiver<T>::recv () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#10 0x76c7b088 in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#11 0x76c79b1c in std::panicking::try::do_call () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#12 0x74eaa2ec in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:80
#13 0x76c7a274 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#14 0x74e7fea4 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#15 0x74ea90f0 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#16 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#17 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#18 0x74c1e494 in start_thread (arg=0x6d7fe1e0) at pthread_create.c:486
#19 0x74d80578 in ?? () at ../sysdeps/unix/sysv/linux/arm/clone.S:73 from /lib/arm-linux-gnueabihf/libc.so.6
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 2 (Thread 0x74b471e0 (LWP 22524)):
#0  futex_wait_cancelable (private=0, expected=0, futex_word=0x4142cc) at ../sysdeps/unix/sysv/linux/futex-internal.h:88
#1  __pthread_cond_wait_common (abstime=0x0, mutex=0x0, cond=0x4142a0) at pthread_cond_wait.c:502
#2  __pthread_cond_wait (cond=0x4142a0, mutex=0x0) at pthread_cond_wait.c:655
#3  0x74e80e98 in std::sys::unix::condvar::Condvar::wait () at src/libstd/sys/unix/condvar.rs:69
#4  std::sys_common::condvar::Condvar::wait () at src/libstd/sys_common/condvar.rs:41
#5  std::sync::condvar::Condvar::wait () at src/libstd/sync/condvar.rs:204
#6  std::thread::park () at src/libstd/thread/mod.rs:911
#7  0x74e964f8 in std::sync::mpsc::blocking::WaitToken::wait () at src/libstd/sync/mpsc/blocking.rs:71
#8  0x75da7c94 in std::sync::mpsc::shared::Packet<T>::recv () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#9  0x75d9b4f8 in std::sync::mpsc::Receiver<T>::recv () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#10 0x75db46f0 in rustc_codegen_ssa::back::write::SharedEmitterMain::check () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#11 0x7040d31c in rustc_codegen_ssa::back::write::OngoingCodegen<B>::join () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#12 0x7044a9d4 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/lib/rustlib/armv7-unknown-linux-gnueabihf/codegen-backends/librustc_codegen_llvm-llvm.so
#13 0x751bb610 in rustc_interface::queries::Query<T>::compute () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#14 0x752826d8 in rustc_interface::queries::<impl rustc_interface::interface::Compiler>::link () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#15 0x750e46e0 in rustc_interface::interface::run_compiler_in_existing_thread_pool () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#16 0x7511a02c in std::thread::local::LocalKey<T>::with () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#17 0x7512e424 in syntax::with_globals () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#18 0x750b9f5c in std::sys_common::backtrace::__rust_begin_short_backtrace () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#19 0x74eaa2ec in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:80
#20 0x750e6870 in core::ops::function::FnOnce::call_once{{vtable-shim}} () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#21 0x74e7fea4 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#22 0x74ea90f0 in <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once () at /rustc/6e0d27d9368e2982bef8e1c4ac14d622c5ad018e/src/liballoc/boxed.rs:770
#23 std::sys_common::thread::start_thread () at src/libstd/sys_common/thread.rs:13
#24 std::sys::unix::thread::Thread::new::thread_start () at src/libstd/sys/unix/thread.rs:79
#25 0x74c1e494 in start_thread (arg=0x74b471e0) at pthread_create.c:486
#26 0x74d80578 in ?? () at ../sysdeps/unix/sysv/linux/arm/clone.S:73 from /lib/arm-linux-gnueabihf/libc.so.6
Backtrace stopped: previous frame identical to this frame (corrupt stack?)

Thread 1 (Thread 0x76ff4020 (LWP 22509)):
#0  0x74c1fa3c in __GI___pthread_timedjoin_ex (threadid=1957982688, thread_return=0x0, abstime=<optimized out>, block=<optimized out>) at pthread_join_common.c:89
#1  0x74ea91c8 in std::sys::unix::thread::Thread::join () at src/libstd/sys/unix/thread.rs:178
#2  0x750e5e9c in std::thread::JoinHandle<T>::join () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#3  0x750dc3d0 in rustc_interface::util::spawn_thread_pool () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#4  0x7513da98 in rustc_driver::run_compiler () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#5  0x75121e84 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#6  0x750de3a8 in std::panicking::try::do_call () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#7  0x74eaa2ec in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:80
#8  0x75143924 in rustc_driver::report_ices_to_stderr_if_any () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#9  0x751440b8 in rustc_driver::main () from /home/fenhl/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/../lib/librustc_driver-b121c769d316ad56.so
#10 0x00400988 in std::rt::lang_start::{{closure}} ()
#11 0x74e9bb98 in std::rt::lang_start_internal::{{closure}} () at src/libstd/rt.rs:49
#12 std::panicking::try::do_call () at src/libstd/panicking.rs:296
#13 0x74eaa2ec in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:80
#14 0x74e9c6c4 in std::panicking::try () at src/libstd/panicking.rs:275
#15 std::panic::catch_unwind () at src/libstd/panic.rs:394
#16 std::rt::lang_start_internal () at src/libstd/rt.rs:48
#17 0x0040096c in main ()
(gdb) 
