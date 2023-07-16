
(gdb) t 2.2
[Switching to thread 2.2 (Thread 0x7fffee1ff700 (LWP 20241))]
#0  0x00007ffff77b207d in backtrace_alloc (state=0x7ffff7fc1000, size=384, error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/mmap.c:115
115 in /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/mmap.c
(gdb) bt
#0  0x00007ffff77b207d in backtrace_alloc (state=0x7ffff7fc1000, size=384, error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/mmap.c:115
#1  0x00007ffff77b2378 in backtrace_vector_grow (state=0x7ffff7fc1000, size=24, error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec=0x7fffee1fa840)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/mmap.c:244
#2  0x00007ffff77ae782 in add_function_range (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, function=0x7fffe47d2fa0, lowpc=140737299864630, highpc=140737299864692, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec=0x7fffee1fa840)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2175
#3  0x00007ffff77aef42 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffee1fa840)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2451
#4  0x00007ffff77af067 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffee1fa940)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2481
#5  0x00007ffff77af067 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffee1faa40)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2481
#6  0x00007ffff77af067 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffe73b61a0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2481
#7  0x00007ffff77aefe4 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffe73b61a0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2467
#8  0x00007ffff77aefe4 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffe73b61a0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2467
#9  0x00007ffff77aefe4 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffe73b61a0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2467
#10 0x00007ffff77aefe4 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffe73b61a0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2467
#11 0x00007ffff77aefe4 in read_function_entry (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, u=0x7fffe7c1a700, base=0, unit_buf=0x7fffee1fb040, lhdr=0x7fffee1fb120, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, vec_function=0x7fffe73b61a0, vec_inlined=0x7fffe73b61a0)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2467
#12 0x00007ffff77af230 in read_function_info (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, lhdr=0x7fffee1fb120, error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, 
    data=0x7fffee1fb4f0, u=0x7fffe7c1a700, fvec=0x7fffe73b61a0, ret_addrs=0x7fffee1fb180, ret_addrs_count=0x7fffee1fb168)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2546
#13 0x00007ffff77af714 in dwarf_lookup_pc (state=0x7ffff7fc1000, ddata=0x7fffe73b6138, pc=140737297503162, callback=0x7ffff7798e60 <std::sys_common::gnu::libbacktrace::print::pcinfo_cb>, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0, found=0x7fffee1fb244)
    at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2731
#14 0x00007ffff77afbc5 in dwarf_fileline (state=0x7ffff7fc1000, pc=140737297503162, callback=0x7ffff7798e60 <std::sys_common::gnu::libbacktrace::print::pcinfo_cb>, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0) at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/dwarf.c:2876
#15 0x00007ffff77b0179 in backtrace_pcinfo (state=0x7ffff7fc1000, pc=140737297503162, callback=0x7ffff7798e60 <std::sys_common::gnu::libbacktrace::print::pcinfo_cb>, 
    error_callback=0x7ffff7798e40 <std::sys_common::gnu::libbacktrace::print::error_cb>, data=0x7fffee1fb4f0) at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libbacktrace/fileline.c:176
#16 0x00007ffff7799808 in std::sys_common::gnu::libbacktrace::print () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys_common/gnu/libbacktrace.rs:166
#17 std::sys::imp::backtrace::tracing::imp::write::trace_fn () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:101
#18 0x00007fffef544679 in _Unwind_Backtrace (trace=0x7ffff7799220 <std::sys::imp::backtrace::tracing::imp::write::trace_fn>, trace_argument=0x7fffee1fb8b0) at /build/gcc-multilib/src/gcc/libgcc/unwind.inc:295
#19 0x00007ffff779917b in std::sys::imp::backtrace::tracing::imp::write () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
#20 0x00007ffff77a8020 in std::panicking::default_hook::{{closure}} () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:247
#21 0x00007ffff77a7bbe in std::panicking::default_hook () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:257
#22 0x00007ffff77a84c8 in std::panicking::rust_panic_with_hook () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:451
#23 0x00007ffff49fd3bb in std::panicking::begin_panic<rustc_errors::FatalError> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:413
#24 0x00007ffff4b8acac in rustc::session::Session::fatal () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/session/mod.rs:191
#25 0x00007ffff7a88f94 in rustc_driver::run::{{closure}}<closure> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:142
