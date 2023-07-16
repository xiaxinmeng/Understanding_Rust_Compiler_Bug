
program received signal EXC_BAD_ACCESS, Could not access memory.
Reason: KERN_INVALID_ADDRESS at address: 0x0000000000000080
[Switching to process 55750 thread 0x271f]
uv__stream_osx_select (arg=0x100d15bd0) at ../../../src/libuv/src/unix/stream.c:157
157   if (fd > s->int_fd)
(gdb) bt
#0  uv__stream_osx_select (arg=0x100d15bd0) at ../../../src/libuv/src/unix/stream.c:157
#1  0x0000000100699b49 in uv__thread_start (arg=<value temporarily unavailable, due to optimizations>) at ../../../src/libuv/src/uv-common.c:274
#2  0x00007fff8fdf9772 in _pthread_start ()
#3  0x00007fff8fde61a1 in thread_start ()
