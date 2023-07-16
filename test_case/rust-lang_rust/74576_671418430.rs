plain
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
In file included from /checkout/src/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_freebsd.cpp:50:
/usr/local/x86_64-unknown-freebsd11/usr/include/sys/timeb.h:42:2: warning: "this file includes <sys/timeb.h> which is deprecated" [-W#warnings]
#warning "this file includes <sys/timeb.h> which is deprecated"
[ 73%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_common.cpp.o
1 warning generated.
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
---
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 95%] Building ASM object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_interceptors_vfork.S.o
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
/checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_vfork.S:12:1: warning: DWARF2 only supports one section per compilation unit
.section .note.GNU-stack,"",%progbits
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 95%] Built target RTSanitizerCommon.x86_64
---
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.h:36:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
1 error generated.
1 error generated.
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_fd.cpp.o] Error 1
make[3]: *** Waiting for unfinished jobs....
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:134: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_fd.cpp.o' failed
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_clock.cpp:13:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
1 error generated.
1 error generated.
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_clock.cpp.o] Error 1
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:62: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_clock.cpp.o' failed
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_java.cpp:14:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutexset.cpp:13:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_external.cpp:12:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:110: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_external.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:302: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_java.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:422: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mutexset.cpp.o' failed
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
#include <sys/_types.h>
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
1 error generated.
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
1 error generated.
1 error generated.
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
1 error generated.
1 error generated.
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_external.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_java.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mutexset.cpp.o] Error 1
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_debugging.cpp:14:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_report.h:15:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_symbolizer.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_vector.h:18:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_vector.h:18:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
1 error generated.
1 error generated.
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_debugging.cpp.o] Error 1
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:86: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_debugging.cpp.o' failed
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_flags.cpp:17:
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
---
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator_internal.h:16:
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
         ^~~~~~~~~~~~~~
1 error generated.
1 error generated.
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface.cpp:15:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:28:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
         ^~~~~~~~~~~~~~
1 error generated.
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_allocator.h:23:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
#include <sys/_types.h>
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_procmaps.h:23:
         ^~~~~~~~~~~~~~
1 error generated.
1 error generated.
1 error generated.
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_linux.h:20:
#include <sys/_types.h>
1 error generated.
1 error generated.
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_platform_limits_freebsd.h:30:10: fatal error: 'sys/_types.h' file not found
#include <sys/_types.h>
1 error generated.
1 error generated.
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_flags.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_ann.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mman.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_atomic.cpp.o] Error 1
make[3]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mutex.cpp.o] Error 1
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:158: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_flags.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:254: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_ann.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:230: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:374: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mman.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:278: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_atomic.cpp.o' failed
lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/build.make:398: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mutex.cpp.o' failed
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
make[2]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/all] Error 2
CMakeFiles/Makefile2:4771: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/all' failed
make[1]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rule] Error 2
CMakeFiles/Makefile2:4783: recipe for target 'lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rule' failed
make: *** [clang_rt.tsan-x86_64] Error 2
Makefile:1711: recipe for target 'clang_rt.tsan-x86_64' failed
command did not execute successfully, got: exit code: 2


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.42/src/lib.rs:861:5
 finished in 25.725
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
Build completed unsuccessfully in 0:07:40
== clock drift check ==
== clock drift check ==
  local time: Mon Aug 10 15:18:04 UTC 2020
  network time: Mon, 10 Aug 2020 15:18:04 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3425) (node)
Terminate orphan process: pid (3453) (python)
