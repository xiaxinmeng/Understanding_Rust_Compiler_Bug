
Program received signal SIGSEGV, Segmentation fault.
[Switching to Thread 5832.0x1738]
0x004ccc9c in stream::read_cb::h0aa53bd142f22e38jZf::v0.11.0.pre ()
(gdb) backtrace
#0  0x004ccc9c in stream::read_cb::h0aa53bd142f22e38jZf::v0.11.0.pre ()
#1  0x004db9ac in uv_process_tcp_read_req (loop=0x8010c8, handle=0x80ae88, req=0x80aebc) at src/win/tcp.c:885
#2  0x004d121e in uv_process_reqs (loop=0x8010c8) at src/win/req-inl.h:152
#3  0x004d1b92 in uv_run (loop=0x8010c8, mode=UV_RUN_DEFAULT) at src/win/core.c:345
#4  0x0047d6ca in uvio::UvEventLoop.EventLoop::run::h1d15421f9e024ae3r1a::v0.11.0.pre ()
#5  0x00452765 in sched::Scheduler::run::ha79c3ff2c3f823d5SWa::v0.11.0.pre ()
#6  0x00451a61 in sched::Scheduler::bootstrap::h85fe42e0b2715604gSa::v0.11.0.pre ()
#7  0x004707a0 in SchedPool::new::closure.6620 ()
#8  0x0047070b in thread::Thread$LT$$LP$$RP$$GT$::start_stack::closure.6611 ()
#9  0x0059a4ff in thread::thread_start::h5ba3e4810fa11878Ead::v0.11.0.pre ()
#10 0x74d5495d in KERNEL32!BaseThreadInitThunk () from C:\WINDOWS\SYSTEM32\kernel32.dll
#11 0x76fb98ee in ntdll!RtlInitializeExceptionChain () from C:\WINDOWS\SYSTEM32\ntdll.dll
#12 0x76fb98c4 in ntdll!RtlInitializeExceptionChain () from C:\WINDOWS\SYSTEM32\ntdll.dll
#13 0x00000000 in ?? ()
