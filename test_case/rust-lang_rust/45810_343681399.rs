c
[ 90%] Built target RTAsan_dynamic.x86_64
CMakeFiles/Makefile2:1636: recipe for target 'lib/asan/CMakeFiles/asan.dir/rule' failed
Makefile:632: recipe for target 'asan' failed

--- stderr
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc: In function ‘void __ubsan::HandleCFIBadType(__ubsan::CFICheckFailData*, __ubsan::ValueHandle, bool, __ubsan::ReportOptions)’:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc:111:15: warning: ‘CheckKindStr’ may be used uninitialized in this function [-Wmaybe-uninitialized]
   const char *CheckKindStr;
               ^~~~~~~~~~~~
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_stoptheworld_linux_libcdep.cc: In function ‘int __sanitizer::TracerThread(void*)’:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_stoptheworld_linux_libcdep.cc:278:22: error: aggregate ‘sigaltstack handler_stack’ has incomplete type and cannot be defined
   struct sigaltstack handler_stack;
                      ^~~~~~~~~~~~~
make[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_stoptheworld_linux_libcdep.cc.o] Error 1
make[3]: *** Waiting for unfinished jobs....
make[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
make[2]: *** Waiting for unfinished jobs....
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_posix.cc: In function ‘__sanitizer::fd_t __sanitizer::OpenFile(const char*, __sanitizer::FileAccessMode, __sanitizer::error_t*)’:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_posix.cc:215:27: warning: ‘flags’ may be used uninitialized in this function [-Wmaybe-uninitialized]
   fd_t res = internal_open(filename, flags, 0660);
              ~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~
In file included from /home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc:265:0:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc: In function ‘__sanitizer::uptr __interceptor_ptrace(int, int, void*, void*)’:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc:2810:21: warning: ‘local_iovec.__sanitizer::__sanitizer_iovec::iov_len’ may be used uninitialized in this function [-Wmaybe-uninitialized]
   __sanitizer_iovec local_iovec;
                     ^~~~~~~~~~~
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc:59:10: warning: ‘local_iovec.__sanitizer::__sanitizer_iovec::iov_base’ may be used uninitialized in this function [-Wmaybe-uninitialized]
     uptr __offset = (uptr)(offset);                                     \
          ^~~~~~~~
In file included from /home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc:265:0:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc:2810:21: note: ‘local_iovec.__sanitizer::__sanitizer_iovec::iov_base’ was declared here
   __sanitizer_iovec local_iovec;
                     ^~~~~~~~~~~
In file included from /home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc:265:0:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc: In function ‘__sanitizer::uptr __interceptor_ptrace(int, int, void*, void*)’:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc:2810:21: warning: ‘local_iovec.__sanitizer::__sanitizer_iovec::iov_len’ may be used uninitialized in this function [-Wmaybe-uninitialized]
   __sanitizer_iovec local_iovec;
                     ^~~~~~~~~~~
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc:59:10: warning: ‘local_iovec.__sanitizer::__sanitizer_iovec::iov_base’ may be used uninitialized in this function [-Wmaybe-uninitialized]
     uptr __offset = (uptr)(offset);                                     \
          ^~~~~~~~
In file included from /home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc:265:0:
/home/simon/rust/src/libcompiler_builtins/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc:2810:21: note: ‘local_iovec.__sanitizer::__sanitizer_iovec::iov_base’ was declared here
   __sanitizer_iovec local_iovec;
                     ^~~~~~~~~~~
make[1]: *** [lib/asan/CMakeFiles/asan.dir/rule] Error 2
make: *** [asan] Error 2
thread 'main' panicked at '
command did not execute successfully, got: exit code: 2

build script failed, must exit now', /home/simon/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.26/src/lib.rs:599:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.

thread 'main' panicked at 'command did not execute successfully: "/home/simon/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "8" "--release" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/home/simon/rust/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src/bootstrap/compile.rs:882:8
