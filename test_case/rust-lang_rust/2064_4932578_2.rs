
==55900== Thread 7: ==55900== Invalid write of size 4 
==55900== at 0x4DAFFB8: uv_buf_init (uv-common.c:75) 
==55900== by 0x8065FF0: rust_uv_buf_init__c_stack_shim (in /home/rustbuild/src/rustbot/workspace-try-x86_64-unknown-linux-gnu/obj/x86_64-unknown-linux-gnu/test/stdtest.stage2-i686-unknown-linux-gnu) 
==55900== by 0x4DAFEBE: ??? (in /home/rustbuild/src/rustbot/workspace-try-x86_64-unknown-linux-gnu/obj/x86_64-unknown-linux-gnu/stage2/lib/rustc/i686-unknown-linux-gnu/lib/librustrt.so) 
==55900== Address 0x8a42efc is 0 bytes after a block of size 12 alloc'd 
==55900== at 0x4D70E8A: malloc (vg_replace_malloc.c:236) 
==55900== by 0x4DAFA6B: memory_region::malloc(unsigned int, char const*, bool) (memory_region.cpp:108) 
==55900== by 0x4DA3D32: rust_kernel::malloc(unsigned int, char const*) (rust_kernel.cpp:47) 
==55900== by 0x4D9E0F6: upcall_s_shared_malloc (rust_upcall.cpp:232) 
==55900== by 0x4DAFEBE: ??? (in /home/rustbuild/src/rustbot/workspace-try-x86_64-unknown-linux-gnu/obj/x86_64-unknown-linux-gnu/stage2/lib/rustc/i686-unknown-linux-gnu/lib/librustrt.so) 
==55900== vex x86->IR: unhandled instruction bytes: 0x2E 0xA4 0x8 0x3 
==55900== valgrind: Unrecognised instruction at address 0x8cac991. 
==55900== Your program just tried to execute an instruction that Valgrind 
==55900== did not recognise. There are two possible reasons for this. 
==55900== 1. Your program has a bug and erroneously jumped to a non-code 
==55900== location. If you are running Memcheck and you just saw a 
==55900== warning about a bad jump, it's probably your program's fault. 
==55900== 2. The instruction is legitimate but Valgrind doesn't handle it, 
==55900== i.e. it's Valgrind's fault. If you think this is the case or 
==55900== you are not sure, please let us know and we'll try to fix it. 
==55900== Either way, Valgrind will now raise a SIGILL signal which will 
==55900== probably kill your program. 
==55900== 
==55900== Process terminating with default action of signal 4 (SIGILL) 
==55900== Illegal opcode at address 0x8CAC991 
==55900== at 0x8CAC991: ??? make: *** [check-stage2-T-i686-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-std-dummy] Killed
