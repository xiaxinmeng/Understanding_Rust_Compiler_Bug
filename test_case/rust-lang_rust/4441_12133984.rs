
compile: rt/x86_64-unknown-linux-gnu/rust_port.o
compile: rt/x86_64-unknown-linux-gnu/rust_upcall.o
In file included from /home/ting/code/rust/src/rt/rust_upcall.cpp:22:
In file included from /home/ting/code/rust/src/rt/rust_unwind.h:24:
/usr/lib/gcc/x86_64-linux-gnu/4.6/include/unwind.h:43:46: error: unknown machine mode
      '__unwind_word__'
typedef unsigned _Unwind_Word __attribute__((__mode__(__unwind_word__)));
                                             ^
/usr/lib/gcc/x86_64-linux-gnu/4.6/include/unwind.h:44:45: error: unknown machine mode
      '__unwind_word__'
typedef signed _Unwind_Sword __attribute__((__mode__(__unwind_word__)));
                                            ^
In file included from /home/ting/code/rust/src/rt/rust_upcall.cpp:22:
/home/ting/code/rust/src/rt/rust_unwind.h:33:14: error: typedef redefinition with different
      types ('void' vs '_Unwind_Exception')
typedef void _Unwind_Exception;
             ^
/usr/lib/gcc/x86_64-linux-gnu/4.6/include/unwind.h:85:8: note: previous definition is here
struct _Unwind_Exception
       ^
3 errors generated.
make: *** [rt/x86_64-unknown-linux-gnu/rust_upcall.o] Error 1
make: *** Waiting for unfinished jobs....
extracting rust-stage0/bin/rustc
extracting rust-stage0/lib/libcore-c3ca5d77d81b46c1-0.5.so
extracting rust-stage0/lib/libstd-4782a756585a81-0.5.so
extracting rust-stage0/lib/librustc-c84825241471686d-0.5.so
extracting rust-stage0/lib/libsyntax-84efebcb12c867a2-0.5.so
extracting rust-stage0/lib/librustrt.so
extracting rust-stage0/lib/librustllvm.so
