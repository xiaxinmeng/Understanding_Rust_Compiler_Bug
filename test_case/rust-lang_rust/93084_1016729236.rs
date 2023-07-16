
Reading symbols from /home/rocket/.cargo/bin/cargo-outdated...
(gdb) r
Starting program: /home/rocket/.cargo/bin/cargo-outdated 
warning: Error disabling address space randomization: Operation not permitted

Program received signal SIGSEGV, Segmentation fault.
0x0000000000000000 in ?? ()
(gdb) bt
#0  0x0000000000000000 in ?? ()
#1  0x00007f673b6e43a0 in ossl_init ()
#2  0x00007f673b6b8db6 in curl_global_init ()
#3  0x00007f673b4b45ca in std::sync::once::Once::call_once::{{closure}} ()
#4  0x00007f673b1e64f8 in std::sync::once::Once::call_inner () at library/std/src/sync/once.rs:419
#5  0x00007f673b1e81af in curl::INIT_CTOR::init_ctor ()
#6  0x00007f673b72c7f8 in libc_start_init () at ../src_musl/src/env/__libc_start_main.c:64
#7  0x00007f673b72c81d in libc_start_main_stage2 () at ../src_musl/src/env/__libc_start_main.c:91
#8  0x00007f673b1e81ca in _start ()
(gdb)
