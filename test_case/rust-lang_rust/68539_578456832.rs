console
% env RUSTFLAGS="-Zsanitizer=memory -Cllvm-args=-msan-track-origins=2" cargo -Zbuild-std build --target x86_64-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
% rust-gdb -q /home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check                                           
Reading symbols from /home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check...done.
(gdb) r
Starting program: /home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
__sanitizer::internal_memset(void*, int, unsigned long) () at /checkout/src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_libc.cc:93
93      /checkout/src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_libc.cc: No such file or directory.
(gdb) quit

 % valgrind -q /home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check 
==11856== Invalid write of size 1
==11856==    at 0x1CFD50: __sanitizer::internal_memset(void*, int, unsigned long) (src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_libc.cc:93)
==11856==    by 0x32601E: std::sys::unix::args::imp::ARGV_INIT_ARRAY::init_wrapper (args.rs:114)
==11856==    by 0x62E1E4: __libc_csu_init (in /home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check)
==11856==    by 0x48C6029: (below main) (libc-start.c:264)
==11856==  Address 0x501ffefffd20 is not stack'd, malloc'd or (recently) free'd
==11856== 
==11856== 
==11856== Process terminating with default action of signal 11 (SIGSEGV)
==11856==  Access not within mapped region at address 0x501FFEFFFD20
==11856==    at 0x1CFD50: __sanitizer::internal_memset(void*, int, unsigned long) (src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_libc.cc:93)
==11856==    by 0x32601E: std::sys::unix::args::imp::ARGV_INIT_ARRAY::init_wrapper (args.rs:114)
==11856==    by 0x62E1E4: __libc_csu_init (in /home/lzutao/.cargo/target_dir/x86_64-unknown-linux-gnu/debug/check)
==11856==    by 0x48C6029: (below main) (libc-start.c:264)
==11856==  If you believe this happened as a result of a stack
==11856==  overflow in your program's main thread (unlikely but
==11856==  possible), you can try to increase the size of the
==11856==  main thread stack using the --main-stacksize= flag.
==11856==  The main thread stack size used in this run was 8388608.
