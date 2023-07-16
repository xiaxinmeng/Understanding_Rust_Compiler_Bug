
(gdb) bt
#0  0x0000000001e904c1 in je_chunk_boot () at /usr/home/andoriyu/Dev/freebsd/ports/lang/rust-nightly/work/rust-4d3eebf/src/jemalloc/src/chunk.c:402
#1  0x0000000001e7f8a9 in malloc_init_hard () at /usr/home/andoriyu/Dev/freebsd/ports/lang/rust-nightly/work/rust-4d3eebf/src/jemalloc/src/jemalloc.c:740
#2  malloc_init () at /usr/home/andoriyu/Dev/freebsd/ports/lang/rust-nightly/work/rust-4d3eebf/src/jemalloc/src/jemalloc.c:314
#3  calloc (num=1, size=1016) at /usr/home/andoriyu/Dev/freebsd/ports/lang/rust-nightly/work/rust-4d3eebf/src/jemalloc/src/jemalloc.c:1130
#4  0x0000000802b0a41a in _thr_alloc (curthread=0x0) at /tank/users/davide/freebsd/lib/libthr/thread/thr_list.c:152
#5  0x0000000802b0b5ea in _libpthread_init (curthread=<optimized out>) at /tank/users/davide/freebsd/lib/libthr/thread/thr_init.c:350
#6  0x0000000802b087aa in _thr_check_init () at /tank/users/davide/freebsd/lib/libthr/thread/thr_private.h:860
#7  __pthread_mutex_lock (mutex=0x803693c48) at /tank/users/davide/freebsd/lib/libthr/thread/thr_mutex.c:439
#8  0x000000080345e6f3 in std::__1::__call_once (flag=@0x8036939d0: 0, arg=0x7fffffffd850, 
    func=0x803435d70 <std::__1::__call_once_proxy<std::__1::tuple<std::__1::(anonymous namespace)::__fake_bind&&> >(void*)>)
    at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/mutex.cpp:250
#9  0x0000000803424c07 in call_once<std::__1::(anonymous namespace)::__fake_bind> (__flag=..., __func=<optimized out>)
    at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/include/mutex:548
#10 __get (this=0x8036939d0 <std::__1::collate<char>::id>) at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:642
#11 install<std::__1::collate<char> > (this=<optimized out>, f=<optimized out>) at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:162
#12 std::__1::locale::__imp::__imp (this=0x803693890, refs=<optimized out>) at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:176
#13 0x0000000803429b94 in make<std::__1::locale::__imp, unsigned int> (a0=1) at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:68
#14 make_classic () at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:463
#15 classic () at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:470
#16 std::__1::locale::__imp::make_global () at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:479
#17 0x0000000803429c98 in __global () at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:486
#18 std::__1::locale::locale (this=0x803691870) at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/locale.cpp:491
#19 0x0000000803452910 in std::__1::basic_streambuf<char, std::__1::char_traits<char> >::basic_streambuf (this=0x803691868)
    at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/include/streambuf:163
#20 0x000000080341307c in std::__1::__stdinbuf<char>::__stdinbuf (this=0x803691868, __fp=0x804057210, __st=0x8036918d0)
    at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/include/__std_stream:44
#21 0x0000000803412c01 in std::__1::ios_base::Init::Init (this=<optimized out>)
    at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/iostream.cpp:49
#22 0x0000000803413029 in ?? () at /tank/users/davide/freebsd/lib/libc++/../../contrib/libc++/src/iostream.cpp:44 from /usr/lib/libc++.so.1
#23 0x000000080346c402 in ?? () from /usr/lib/libc++.so.1
#24 0x00000008026ec800 in ?? ()
#25 0x00007fffffffe2d8 in ?? ()
#26 0x00007fffffffe260 in ?? ()
#27 0x000000080340f5c6 in _init () from /usr/lib/libc++.so.1
#28 0x00007fffffffe260 in ?? ()
#29 0x00000008026c7d6d in objlist_call_init (list=<optimized out>, lockstate=0x8026ec800) at /tank/users/davide/freebsd/libexec/rtld-elf/rtld.c:2492
