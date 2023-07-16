
(gdb) bt
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007ffff77a7859 in __GI_abort () at abort.c:79
#2  0x00007ffff781229e in __libc_message (action=action@entry=do_abort, fmt=fmt@entry=0x7ffff793c298 "%s\n") at ../sysdeps/posix/libc_fatal.c:155
#3  0x00007ffff781a32c in malloc_printerr (str=str@entry=0x7ffff793a4c1 "free(): invalid pointer") at malloc.c:5347
#4  0x00007ffff781bb5c in _int_free (av=<optimised out>, p=<optimised out>, have_lock=0) at malloc.c:4173
#5  0x00000000010bad4e in __gnu_cxx::new_allocator<Rust::Compile::fncontext>::deallocate (this=0x7fffffffd578, __p=0x45af0d0) at /usr/include/c++/9/ext/new_allocator.h:128
#6  0x00000000010b9150 in std::allocator_traits<std::allocator<Rust::Compile::fncontext> >::deallocate (__a=..., __p=0x45af0d0, __n=1) at /usr/include/c++/9/bits/alloc_traits.h:469
#7  0x00000000010b6826 in std::_Vector_base<Rust::Compile::fncontext, std::allocator<Rust::Compile::fncontext> >::_M_deallocate (this=0x7fffffffd578, __p=0x45af0d0, __n=1) at /usr/include/c++/9/bits/stl_vector.h:351
#8  0x00000000010b38ea in std::_Vector_base<Rust::Compile::fncontext, std::allocator<Rust::Compile::fncontext> >::~_Vector_base (this=0x7fffffffd578, __in_chrg=<optimised out>) at /usr/include/c++/9/bits/stl_vector.h:332
#9  0x00000000010b393f in std::vector<Rust::Compile::fncontext, std::allocator<Rust::Compile::fncontext> >::~vector (this=0x7fffffffd578, __in_chrg=<optimised out>) at /usr/include/c++/9/bits/stl_vector.h:680
#10 0x00000000010b2238 in Rust::Compile::Context::~Context (this=0x7fffffffd520, __in_chrg=<optimised out>) at ../../gccrs/gcc/rust/backend/rust-compile-context.h:41
#11 0x00000000010a9698 in Rust::Session::parse_file (this=0x438f5e0 <Rust::Session::get_instance()::instance>, filename=0x7fffffffdf63 "test.rs") at ../../gccrs/gcc/rust/rust-session-manager.cc:603
#12 0x00000000010a9161 in Rust::Session::parse_files (this=0x438f5e0 <Rust::Session::get_instance()::instance>, num_files=1, files=0x4584e60) at ../../gccrs/gcc/rust/rust-session-manager.cc:483
#13 0x0000000000f7325c in grs_langhook_parse_file () at ../../gccrs/gcc/rust/rust-lang.cc:171
#14 0x0000000001a92ed6 in compile_file () at ../../gccrs/gcc/toplev.cc:452
#15 0x0000000001a960d8 in do_compile (no_backend=false) at ../../gccrs/gcc/toplev.cc:2158
#16 0x0000000001a964de in toplev::main (this=0x7fffffffd9e2, argc=24, argv=0x7fffffffdaf8) at ../../gccrs/gcc/toplev.cc:2310
#17 0x000000000315e297 in main (argc=24, argv=0x7fffffffdaf8) at ../../gccrs/gcc/main.cc:39
