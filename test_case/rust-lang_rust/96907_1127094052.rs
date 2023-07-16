plain
 ---> 43f9e80d631e
Step 6/48 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
 ---> Running in 5cde6b30aedf
Warning: apt-key output should not be parsed (stdout is not a terminal)
Executing: /tmp/apt-key-gpghome.SayP19sj8W/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
gpg: Total number processed: 1
gpg:               imported: 1
Removing intermediate container 5cde6b30aedf
 ---> 765b65489aad
---
+ export PATH=/tmp/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
+ git clone https://github.com/WebAssembly/wasi-libc
Cloning into 'wasi-libc'...
+ cd wasi-libc
+ git reset --hard 9886d3d6200fcc3726329966860fc058707406cd
HEAD is now at 9886d3d Fix `find_relpath` to handle large buffers correctly.
+ make -j16 INSTALL_DIR=/wasm32-wasi install
find: '/tmp/wasi-libc/build': No such file or directory
#
# Install the include files.
---
rm -f /tmp/wasi-libc/sysroot/include/"bits/syscall.h.in" /tmp/wasi-libc/sysroot/include/"bits/alltypes.h.in" /tmp/wasi-libc/sysroot/include/"alltypes.h.in" /tmp/wasi-libc/sysroot/include/"stdarg.h" /tmp/wasi-libc/sysroot/include/"stddef.h" /tmp/wasi-libc/sysroot/include/"bits/errno.h" /tmp/wasi-libc/sysroot/include/"sys/procfs.h" /tmp/wasi-libc/sysroot/include/"sys/user.h" /tmp/wasi-libc/sysroot/include/"sys/kd.h" /tmp/wasi-libc/sysroot/include/"sys/vt.h" /tmp/wasi-libc/sysroot/include/"sys/soundcard.h" /tmp/wasi-libc/sysroot/include/"sys/sem.h" /tmp/wasi-libc/sysroot/include/"sys/shm.h" /tmp/wasi-libc/sysroot/include/"sys/msg.h" /tmp/wasi-libc/sysroot/include/"sys/ipc.h" /tmp/wasi-libc/sysroot/include/"sys/ptrace.h" /tmp/wasi-libc/sysroot/include/"sys/statfs.h" /tmp/wasi-libc/sysroot/include/"bits/kd.h" /tmp/wasi-libc/sysroot/include/"bits/vt.h" /tmp/wasi-libc/sysroot/include/"bits/soundcard.h" /tmp/wasi-libc/sysroot/include/"bits/sem.h" /tmp/wasi-libc/sysroot/include/"bits/shm.h" /tmp/wasi-libc/sysroot/include/"bits/msg.h" /tmp/wasi-libc/sysroot/include/"bits/ipc.h" /tmp/wasi-libc/sysroot/include/"bits/ptrace.h" /tmp/wasi-libc/sysroot/include/"bits/statfs.h" /tmp/wasi-libc/sysroot/include/"sys/vfs.h" /tmp/wasi-libc/sysroot/include/"sys/statvfs.h" /tmp/wasi-libc/sysroot/include/"syslog.h" /tmp/wasi-libc/sysroot/include/"sys/syslog.h" /tmp/wasi-libc/sysroot/include/"wait.h" /tmp/wasi-libc/sysroot/include/"sys/wait.h" /tmp/wasi-libc/sysroot/include/"ucontext.h" /tmp/wasi-libc/sysroot/include/"sys/ucontext.h" /tmp/wasi-libc/sysroot/include/"paths.h" /tmp/wasi-libc/sysroot/include/"utmp.h" /tmp/wasi-libc/sysroot/include/"utmpx.h" /tmp/wasi-libc/sysroot/include/"lastlog.h" /tmp/wasi-libc/sysroot/include/"sys/acct.h" /tmp/wasi-libc/sysroot/include/"sys/cachectl.h" /tmp/wasi-libc/sysroot/include/"sys/epoll.h" /tmp/wasi-libc/sysroot/include/"sys/reboot.h" /tmp/wasi-libc/sysroot/include/"sys/swap.h" /tmp/wasi-libc/sysroot/include/"sys/sendfile.h" /tmp/wasi-libc/sysroot/include/"sys/inotify.h" /tmp/wasi-libc/sysroot/include/"sys/quota.h" /tmp/wasi-libc/sysroot/include/"sys/klog.h" /tmp/wasi-libc/sysroot/include/"sys/fsuid.h" /tmp/wasi-libc/sysroot/include/"sys/io.h" /tmp/wasi-libc/sysroot/include/"sys/prctl.h" /tmp/wasi-libc/sysroot/include/"sys/mtio.h" /tmp/wasi-libc/sysroot/include/"sys/mount.h" /tmp/wasi-libc/sysroot/include/"sys/fanotify.h" /tmp/wasi-libc/sysroot/include/"sys/personality.h" /tmp/wasi-libc/sysroot/include/"elf.h" /tmp/wasi-libc/sysroot/include/"link.h" /tmp/wasi-libc/sysroot/include/"bits/link.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi_ioctl.h" /tmp/wasi-libc/sysroot/include/"scsi/sg.h" /tmp/wasi-libc/sysroot/include/"sys/auxv.h" /tmp/wasi-libc/sysroot/include/"pwd.h" /tmp/wasi-libc/sysroot/include/"shadow.h" /tmp/wasi-libc/sysroot/include/"grp.h" /tmp/wasi-libc/sysroot/include/"mntent.h" /tmp/wasi-libc/sysroot/include/"netdb.h" /tmp/wasi-libc/sysroot/include/"resolv.h" /tmp/wasi-libc/sysroot/include/"pty.h" /tmp/wasi-libc/sysroot/include/"dlfcn.h" /tmp/wasi-libc/sysroot/include/"setjmp.h" /tmp/wasi-libc/sysroot/include/"ulimit.h" /tmp/wasi-libc/sysroot/include/"sys/xattr.h" /tmp/wasi-libc/sysroot/include/"wordexp.h" /tmp/wasi-libc/sysroot/include/"spawn.h" /tmp/wasi-libc/sysroot/include/"sys/membarrier.h" /tmp/wasi-libc/sysroot/include/"sys/signalfd.h" /tmp/wasi-libc/sysroot/include/"termios.h" /tmp/wasi-libc/sysroot/include/"sys/termios.h" /tmp/wasi-libc/sysroot/include/"bits/termios.h" /tmp/wasi-libc/sysroot/include/"net/if.h" /tmp/wasi-libc/sysroot/include/"net/if_arp.h" /tmp/wasi-libc/sysroot/include/"net/ethernet.h" /tmp/wasi-libc/sysroot/include/"net/route.h" /tmp/wasi-libc/sysroot/include/"netinet/if_ether.h" /tmp/wasi-libc/sysroot/include/"netinet/ether.h" /tmp/wasi-libc/sysroot/include/"sys/timerfd.h" /tmp/wasi-libc/sysroot/include/"libintl.h" /tmp/wasi-libc/sysroot/include/"sys/sysmacros.h" /tmp/wasi-libc/sysroot/include/"aio.h" /tmp/wasi-libc/sysroot/include/"pthread.h"
#
# Build the startup files.
#
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/dlmalloc/include -MD -MP -o /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o -c /tmp/wasi-libc/dlmalloc/src/dlmalloc.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
cd "/tmp/wasi-libc/build" && \
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -c /tmp/wasi-libc/libc-bottom-half/crt/crt1.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-reactor.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-command.c -MD -MP && \
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.c
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.c
make: *** [Makefile:400: /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o] Error 1
make: *** Waiting for unfinished jobs....
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.c
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o] Error 1
make: *** [Makefile:457: startup_files] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o] Error 1
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon    513kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
+ export PATH=/tmp/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
+ git clone https://github.com/WebAssembly/wasi-libc
Cloning into 'wasi-libc'...
+ cd wasi-libc
+ git reset --hard 9886d3d6200fcc3726329966860fc058707406cd
HEAD is now at 9886d3d Fix `find_relpath` to handle large buffers correctly.
+ make -j16 INSTALL_DIR=/wasm32-wasi install
find: '/tmp/wasi-libc/build': No such file or directory
#
# Install the include files.
---
# Remove selected header files.
rm -f /tmp/wasi-libc/sysroot/include/"bits/syscall.h.in" /tmp/wasi-libc/sysroot/include/"bits/alltypes.h.in" /tmp/wasi-libc/sysroot/include/"alltypes.h.in" /tmp/wasi-libc/sysroot/include/"stdarg.h" /tmp/wasi-libc/sysroot/include/"stddef.h" /tmp/wasi-libc/sysroot/include/"bits/errno.h" /tmp/wasi-libc/sysroot/include/"sys/procfs.h" /tmp/wasi-libc/sysroot/include/"sys/user.h" /tmp/wasi-libc/sysroot/include/"sys/kd.h" /tmp/wasi-libc/sysroot/include/"sys/vt.h" /tmp/wasi-libc/sysroot/include/"sys/soundcard.h" /tmp/wasi-libc/sysroot/include/"sys/sem.h" /tmp/wasi-libc/sysroot/include/"sys/shm.h" /tmp/wasi-libc/sysroot/include/"sys/msg.h" /tmp/wasi-libc/sysroot/include/"sys/ipc.h" /tmp/wasi-libc/sysroot/include/"sys/ptrace.h" /tmp/wasi-libc/sysroot/include/"sys/statfs.h" /tmp/wasi-libc/sysroot/include/"bits/kd.h" /tmp/wasi-libc/sysroot/include/"bits/vt.h" /tmp/wasi-libc/sysroot/include/"bits/soundcard.h" /tmp/wasi-libc/sysroot/include/"bits/sem.h" /tmp/wasi-libc/sysroot/include/"bits/shm.h" /tmp/wasi-libc/sysroot/include/"bits/msg.h" /tmp/wasi-libc/sysroot/include/"bits/ipc.h" /tmp/wasi-libc/sysroot/include/"bits/ptrace.h" /tmp/wasi-libc/sysroot/include/"bits/statfs.h" /tmp/wasi-libc/sysroot/include/"sys/vfs.h" /tmp/wasi-libc/sysroot/include/"sys/statvfs.h" /tmp/wasi-libc/sysroot/include/"syslog.h" /tmp/wasi-libc/sysroot/include/"sys/syslog.h" /tmp/wasi-libc/sysroot/include/"wait.h" /tmp/wasi-libc/sysroot/include/"sys/wait.h" /tmp/wasi-libc/sysroot/include/"ucontext.h" /tmp/wasi-libc/sysroot/include/"sys/ucontext.h" /tmp/wasi-libc/sysroot/include/"paths.h" /tmp/wasi-libc/sysroot/include/"utmp.h" /tmp/wasi-libc/sysroot/include/"utmpx.h" /tmp/wasi-libc/sysroot/include/"lastlog.h" /tmp/wasi-libc/sysroot/include/"sys/acct.h" /tmp/wasi-libc/sysroot/include/"sys/cachectl.h" /tmp/wasi-libc/sysroot/include/"sys/epoll.h" /tmp/wasi-libc/sysroot/include/"sys/reboot.h" /tmp/wasi-libc/sysroot/include/"sys/swap.h" /tmp/wasi-libc/sysroot/include/"sys/sendfile.h" /tmp/wasi-libc/sysroot/include/"sys/inotify.h" /tmp/wasi-libc/sysroot/include/"sys/quota.h" /tmp/wasi-libc/sysroot/include/"sys/klog.h" /tmp/wasi-libc/sysroot/include/"sys/fsuid.h" /tmp/wasi-libc/sysroot/include/"sys/io.h" /tmp/wasi-libc/sysroot/include/"sys/prctl.h" /tmp/wasi-libc/sysroot/include/"sys/mtio.h" /tmp/wasi-libc/sysroot/include/"sys/mount.h" /tmp/wasi-libc/sysroot/include/"sys/fanotify.h" /tmp/wasi-libc/sysroot/include/"sys/personality.h" /tmp/wasi-libc/sysroot/include/"elf.h" /tmp/wasi-libc/sysroot/include/"link.h" /tmp/wasi-libc/sysroot/include/"bits/link.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi_ioctl.h" /tmp/wasi-libc/sysroot/include/"scsi/sg.h" /tmp/wasi-libc/sysroot/include/"sys/auxv.h" /tmp/wasi-libc/sysroot/include/"pwd.h" /tmp/wasi-libc/sysroot/include/"shadow.h" /tmp/wasi-libc/sysroot/include/"grp.h" /tmp/wasi-libc/sysroot/include/"mntent.h" /tmp/wasi-libc/sysroot/include/"netdb.h" /tmp/wasi-libc/sysroot/include/"resolv.h" /tmp/wasi-libc/sysroot/include/"pty.h" /tmp/wasi-libc/sysroot/include/"dlfcn.h" /tmp/wasi-libc/sysroot/include/"setjmp.h" /tmp/wasi-libc/sysroot/include/"ulimit.h" /tmp/wasi-libc/sysroot/include/"sys/xattr.h" /tmp/wasi-libc/sysroot/include/"wordexp.h" /tmp/wasi-libc/sysroot/include/"spawn.h" /tmp/wasi-libc/sysroot/include/"sys/membarrier.h" /tmp/wasi-libc/sysroot/include/"sys/signalfd.h" /tmp/wasi-libc/sysroot/include/"termios.h" /tmp/wasi-libc/sysroot/include/"sys/termios.h" /tmp/wasi-libc/sysroot/include/"bits/termios.h" /tmp/wasi-libc/sysroot/include/"net/if.h" /tmp/wasi-libc/sysroot/include/"net/if_arp.h" /tmp/wasi-libc/sysroot/include/"net/ethernet.h" /tmp/wasi-libc/sysroot/include/"net/route.h" /tmp/wasi-libc/sysroot/include/"netinet/if_ether.h" /tmp/wasi-libc/sysroot/include/"netinet/ether.h" /tmp/wasi-libc/sysroot/include/"sys/timerfd.h" /tmp/wasi-libc/sysroot/include/"libintl.h" /tmp/wasi-libc/sysroot/include/"sys/sysmacros.h" /tmp/wasi-libc/sysroot/include/"aio.h" /tmp/wasi-libc/sysroot/include/"pthread.h"
#
# Build the startup files.
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/dlmalloc/include -MD -MP -o /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o -c /tmp/wasi-libc/dlmalloc/src/dlmalloc.c
#
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.c
gcc-8: error: single: No such file or directory
cd "/tmp/wasi-libc/build" && \
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -c /tmp/wasi-libc/libc-bottom-half/crt/crt1.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-reactor.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-command.c -MD -MP && \
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.c
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o] Error 1
make: *** Waiting for unfinished jobs....
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o] Error 1
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.c
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:457: startup_files] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o] Error 1
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/mkdirat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/mkdirat.c
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o] Error 1
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/mkdirat.o] Error 1
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon    513kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
+ export PATH=/tmp/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
+ git clone https://github.com/WebAssembly/wasi-libc
Cloning into 'wasi-libc'...
+ cd wasi-libc
+ git reset --hard 9886d3d6200fcc3726329966860fc058707406cd
HEAD is now at 9886d3d Fix `find_relpath` to handle large buffers correctly.
+ make -j16 INSTALL_DIR=/wasm32-wasi install
find: '/tmp/wasi-libc/build': No such file or directory
#
# Install the include files.
---
rm -f /tmp/wasi-libc/sysroot/include/"bits/syscall.h.in" /tmp/wasi-libc/sysroot/include/"bits/alltypes.h.in" /tmp/wasi-libc/sysroot/include/"alltypes.h.in" /tmp/wasi-libc/sysroot/include/"stdarg.h" /tmp/wasi-libc/sysroot/include/"stddef.h" /tmp/wasi-libc/sysroot/include/"bits/errno.h" /tmp/wasi-libc/sysroot/include/"sys/procfs.h" /tmp/wasi-libc/sysroot/include/"sys/user.h" /tmp/wasi-libc/sysroot/include/"sys/kd.h" /tmp/wasi-libc/sysroot/include/"sys/vt.h" /tmp/wasi-libc/sysroot/include/"sys/soundcard.h" /tmp/wasi-libc/sysroot/include/"sys/sem.h" /tmp/wasi-libc/sysroot/include/"sys/shm.h" /tmp/wasi-libc/sysroot/include/"sys/msg.h" /tmp/wasi-libc/sysroot/include/"sys/ipc.h" /tmp/wasi-libc/sysroot/include/"sys/ptrace.h" /tmp/wasi-libc/sysroot/include/"sys/statfs.h" /tmp/wasi-libc/sysroot/include/"bits/kd.h" /tmp/wasi-libc/sysroot/include/"bits/vt.h" /tmp/wasi-libc/sysroot/include/"bits/soundcard.h" /tmp/wasi-libc/sysroot/include/"bits/sem.h" /tmp/wasi-libc/sysroot/include/"bits/shm.h" /tmp/wasi-libc/sysroot/include/"bits/msg.h" /tmp/wasi-libc/sysroot/include/"bits/ipc.h" /tmp/wasi-libc/sysroot/include/"bits/ptrace.h" /tmp/wasi-libc/sysroot/include/"bits/statfs.h" /tmp/wasi-libc/sysroot/include/"sys/vfs.h" /tmp/wasi-libc/sysroot/include/"sys/statvfs.h" /tmp/wasi-libc/sysroot/include/"syslog.h" /tmp/wasi-libc/sysroot/include/"sys/syslog.h" /tmp/wasi-libc/sysroot/include/"wait.h" /tmp/wasi-libc/sysroot/include/"sys/wait.h" /tmp/wasi-libc/sysroot/include/"ucontext.h" /tmp/wasi-libc/sysroot/include/"sys/ucontext.h" /tmp/wasi-libc/sysroot/include/"paths.h" /tmp/wasi-libc/sysroot/include/"utmp.h" /tmp/wasi-libc/sysroot/include/"utmpx.h" /tmp/wasi-libc/sysroot/include/"lastlog.h" /tmp/wasi-libc/sysroot/include/"sys/acct.h" /tmp/wasi-libc/sysroot/include/"sys/cachectl.h" /tmp/wasi-libc/sysroot/include/"sys/epoll.h" /tmp/wasi-libc/sysroot/include/"sys/reboot.h" /tmp/wasi-libc/sysroot/include/"sys/swap.h" /tmp/wasi-libc/sysroot/include/"sys/sendfile.h" /tmp/wasi-libc/sysroot/include/"sys/inotify.h" /tmp/wasi-libc/sysroot/include/"sys/quota.h" /tmp/wasi-libc/sysroot/include/"sys/klog.h" /tmp/wasi-libc/sysroot/include/"sys/fsuid.h" /tmp/wasi-libc/sysroot/include/"sys/io.h" /tmp/wasi-libc/sysroot/include/"sys/prctl.h" /tmp/wasi-libc/sysroot/include/"sys/mtio.h" /tmp/wasi-libc/sysroot/include/"sys/mount.h" /tmp/wasi-libc/sysroot/include/"sys/fanotify.h" /tmp/wasi-libc/sysroot/include/"sys/personality.h" /tmp/wasi-libc/sysroot/include/"elf.h" /tmp/wasi-libc/sysroot/include/"link.h" /tmp/wasi-libc/sysroot/include/"bits/link.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi_ioctl.h" /tmp/wasi-libc/sysroot/include/"scsi/sg.h" /tmp/wasi-libc/sysroot/include/"sys/auxv.h" /tmp/wasi-libc/sysroot/include/"pwd.h" /tmp/wasi-libc/sysroot/include/"shadow.h" /tmp/wasi-libc/sysroot/include/"grp.h" /tmp/wasi-libc/sysroot/include/"mntent.h" /tmp/wasi-libc/sysroot/include/"netdb.h" /tmp/wasi-libc/sysroot/include/"resolv.h" /tmp/wasi-libc/sysroot/include/"pty.h" /tmp/wasi-libc/sysroot/include/"dlfcn.h" /tmp/wasi-libc/sysroot/include/"setjmp.h" /tmp/wasi-libc/sysroot/include/"ulimit.h" /tmp/wasi-libc/sysroot/include/"sys/xattr.h" /tmp/wasi-libc/sysroot/include/"wordexp.h" /tmp/wasi-libc/sysroot/include/"spawn.h" /tmp/wasi-libc/sysroot/include/"sys/membarrier.h" /tmp/wasi-libc/sysroot/include/"sys/signalfd.h" /tmp/wasi-libc/sysroot/include/"termios.h" /tmp/wasi-libc/sysroot/include/"sys/termios.h" /tmp/wasi-libc/sysroot/include/"bits/termios.h" /tmp/wasi-libc/sysroot/include/"net/if.h" /tmp/wasi-libc/sysroot/include/"net/if_arp.h" /tmp/wasi-libc/sysroot/include/"net/ethernet.h" /tmp/wasi-libc/sysroot/include/"net/route.h" /tmp/wasi-libc/sysroot/include/"netinet/if_ether.h" /tmp/wasi-libc/sysroot/include/"netinet/ether.h" /tmp/wasi-libc/sysroot/include/"sys/timerfd.h" /tmp/wasi-libc/sysroot/include/"libintl.h" /tmp/wasi-libc/sysroot/include/"sys/sysmacros.h" /tmp/wasi-libc/sysroot/include/"aio.h" /tmp/wasi-libc/sysroot/include/"pthread.h"
#
# Build the startup files.
#
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/dlmalloc/include -MD -MP -o /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o -c /tmp/wasi-libc/dlmalloc/src/dlmalloc.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.c
gcc-8: error: single: No such file or directory
cd "/tmp/wasi-libc/build" && \
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -c /tmp/wasi-libc/libc-bottom-half/crt/crt1.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-reactor.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-command.c -MD -MP && \
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:399: /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o] Error 1
make: *** Waiting for unfinished jobs....
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o] Error 1
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.c
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.c
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o] Error 1
make: *** [Makefile:457: startup_files] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: single: No such file or directory
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o] Error 1
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o] Error 1
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon    513kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
+ export PATH=/tmp/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
+ git clone https://github.com/WebAssembly/wasi-libc
Cloning into 'wasi-libc'...
+ cd wasi-libc
+ git reset --hard 9886d3d6200fcc3726329966860fc058707406cd
HEAD is now at 9886d3d Fix `find_relpath` to handle large buffers correctly.
+ make -j16 INSTALL_DIR=/wasm32-wasi install
find: '/tmp/wasi-libc/build': No such file or directory
#
# Install the include files.
---
rm -f /tmp/wasi-libc/sysroot/include/"bits/syscall.h.in" /tmp/wasi-libc/sysroot/include/"bits/alltypes.h.in" /tmp/wasi-libc/sysroot/include/"alltypes.h.in" /tmp/wasi-libc/sysroot/include/"stdarg.h" /tmp/wasi-libc/sysroot/include/"stddef.h" /tmp/wasi-libc/sysroot/include/"bits/errno.h" /tmp/wasi-libc/sysroot/include/"sys/procfs.h" /tmp/wasi-libc/sysroot/include/"sys/user.h" /tmp/wasi-libc/sysroot/include/"sys/kd.h" /tmp/wasi-libc/sysroot/include/"sys/vt.h" /tmp/wasi-libc/sysroot/include/"sys/soundcard.h" /tmp/wasi-libc/sysroot/include/"sys/sem.h" /tmp/wasi-libc/sysroot/include/"sys/shm.h" /tmp/wasi-libc/sysroot/include/"sys/msg.h" /tmp/wasi-libc/sysroot/include/"sys/ipc.h" /tmp/wasi-libc/sysroot/include/"sys/ptrace.h" /tmp/wasi-libc/sysroot/include/"sys/statfs.h" /tmp/wasi-libc/sysroot/include/"bits/kd.h" /tmp/wasi-libc/sysroot/include/"bits/vt.h" /tmp/wasi-libc/sysroot/include/"bits/soundcard.h" /tmp/wasi-libc/sysroot/include/"bits/sem.h" /tmp/wasi-libc/sysroot/include/"bits/shm.h" /tmp/wasi-libc/sysroot/include/"bits/msg.h" /tmp/wasi-libc/sysroot/include/"bits/ipc.h" /tmp/wasi-libc/sysroot/include/"bits/ptrace.h" /tmp/wasi-libc/sysroot/include/"bits/statfs.h" /tmp/wasi-libc/sysroot/include/"sys/vfs.h" /tmp/wasi-libc/sysroot/include/"sys/statvfs.h" /tmp/wasi-libc/sysroot/include/"syslog.h" /tmp/wasi-libc/sysroot/include/"sys/syslog.h" /tmp/wasi-libc/sysroot/include/"wait.h" /tmp/wasi-libc/sysroot/include/"sys/wait.h" /tmp/wasi-libc/sysroot/include/"ucontext.h" /tmp/wasi-libc/sysroot/include/"sys/ucontext.h" /tmp/wasi-libc/sysroot/include/"paths.h" /tmp/wasi-libc/sysroot/include/"utmp.h" /tmp/wasi-libc/sysroot/include/"utmpx.h" /tmp/wasi-libc/sysroot/include/"lastlog.h" /tmp/wasi-libc/sysroot/include/"sys/acct.h" /tmp/wasi-libc/sysroot/include/"sys/cachectl.h" /tmp/wasi-libc/sysroot/include/"sys/epoll.h" /tmp/wasi-libc/sysroot/include/"sys/reboot.h" /tmp/wasi-libc/sysroot/include/"sys/swap.h" /tmp/wasi-libc/sysroot/include/"sys/sendfile.h" /tmp/wasi-libc/sysroot/include/"sys/inotify.h" /tmp/wasi-libc/sysroot/include/"sys/quota.h" /tmp/wasi-libc/sysroot/include/"sys/klog.h" /tmp/wasi-libc/sysroot/include/"sys/fsuid.h" /tmp/wasi-libc/sysroot/include/"sys/io.h" /tmp/wasi-libc/sysroot/include/"sys/prctl.h" /tmp/wasi-libc/sysroot/include/"sys/mtio.h" /tmp/wasi-libc/sysroot/include/"sys/mount.h" /tmp/wasi-libc/sysroot/include/"sys/fanotify.h" /tmp/wasi-libc/sysroot/include/"sys/personality.h" /tmp/wasi-libc/sysroot/include/"elf.h" /tmp/wasi-libc/sysroot/include/"link.h" /tmp/wasi-libc/sysroot/include/"bits/link.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi_ioctl.h" /tmp/wasi-libc/sysroot/include/"scsi/sg.h" /tmp/wasi-libc/sysroot/include/"sys/auxv.h" /tmp/wasi-libc/sysroot/include/"pwd.h" /tmp/wasi-libc/sysroot/include/"shadow.h" /tmp/wasi-libc/sysroot/include/"grp.h" /tmp/wasi-libc/sysroot/include/"mntent.h" /tmp/wasi-libc/sysroot/include/"netdb.h" /tmp/wasi-libc/sysroot/include/"resolv.h" /tmp/wasi-libc/sysroot/include/"pty.h" /tmp/wasi-libc/sysroot/include/"dlfcn.h" /tmp/wasi-libc/sysroot/include/"setjmp.h" /tmp/wasi-libc/sysroot/include/"ulimit.h" /tmp/wasi-libc/sysroot/include/"sys/xattr.h" /tmp/wasi-libc/sysroot/include/"wordexp.h" /tmp/wasi-libc/sysroot/include/"spawn.h" /tmp/wasi-libc/sysroot/include/"sys/membarrier.h" /tmp/wasi-libc/sysroot/include/"sys/signalfd.h" /tmp/wasi-libc/sysroot/include/"termios.h" /tmp/wasi-libc/sysroot/include/"sys/termios.h" /tmp/wasi-libc/sysroot/include/"bits/termios.h" /tmp/wasi-libc/sysroot/include/"net/if.h" /tmp/wasi-libc/sysroot/include/"net/if_arp.h" /tmp/wasi-libc/sysroot/include/"net/ethernet.h" /tmp/wasi-libc/sysroot/include/"net/route.h" /tmp/wasi-libc/sysroot/include/"netinet/if_ether.h" /tmp/wasi-libc/sysroot/include/"netinet/ether.h" /tmp/wasi-libc/sysroot/include/"sys/timerfd.h" /tmp/wasi-libc/sysroot/include/"libintl.h" /tmp/wasi-libc/sysroot/include/"sys/sysmacros.h" /tmp/wasi-libc/sysroot/include/"aio.h" /tmp/wasi-libc/sysroot/include/"pthread.h"
#
# Build the startup files.
#
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/dlmalloc/include -MD -MP -o /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o -c /tmp/wasi-libc/dlmalloc/src/dlmalloc.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.c
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.c
cd "/tmp/wasi-libc/build" && \
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -c /tmp/wasi-libc/libc-bottom-half/crt/crt1.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-reactor.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-command.c -MD -MP && \
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:399: /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
make: *** Waiting for unfinished jobs....
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o] Error 1
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.c
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.c
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.c
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.c
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/mkdirat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/mkdirat.c
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o] Error 1
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: single: No such file or directory
make: *** [Makefile:457: startup_files] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o] Error 1
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/mkdirat.o] Error 1
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon    513kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
+ export PATH=/tmp/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
+ git clone https://github.com/WebAssembly/wasi-libc
Cloning into 'wasi-libc'...
+ cd wasi-libc
+ git reset --hard 9886d3d6200fcc3726329966860fc058707406cd
HEAD is now at 9886d3d Fix `find_relpath` to handle large buffers correctly.
+ make -j16 INSTALL_DIR=/wasm32-wasi install
find: '/tmp/wasi-libc/build': No such file or directory
#
# Install the include files.
---
rm -f /tmp/wasi-libc/sysroot/include/"bits/syscall.h.in" /tmp/wasi-libc/sysroot/include/"bits/alltypes.h.in" /tmp/wasi-libc/sysroot/include/"alltypes.h.in" /tmp/wasi-libc/sysroot/include/"stdarg.h" /tmp/wasi-libc/sysroot/include/"stddef.h" /tmp/wasi-libc/sysroot/include/"bits/errno.h" /tmp/wasi-libc/sysroot/include/"sys/procfs.h" /tmp/wasi-libc/sysroot/include/"sys/user.h" /tmp/wasi-libc/sysroot/include/"sys/kd.h" /tmp/wasi-libc/sysroot/include/"sys/vt.h" /tmp/wasi-libc/sysroot/include/"sys/soundcard.h" /tmp/wasi-libc/sysroot/include/"sys/sem.h" /tmp/wasi-libc/sysroot/include/"sys/shm.h" /tmp/wasi-libc/sysroot/include/"sys/msg.h" /tmp/wasi-libc/sysroot/include/"sys/ipc.h" /tmp/wasi-libc/sysroot/include/"sys/ptrace.h" /tmp/wasi-libc/sysroot/include/"sys/statfs.h" /tmp/wasi-libc/sysroot/include/"bits/kd.h" /tmp/wasi-libc/sysroot/include/"bits/vt.h" /tmp/wasi-libc/sysroot/include/"bits/soundcard.h" /tmp/wasi-libc/sysroot/include/"bits/sem.h" /tmp/wasi-libc/sysroot/include/"bits/shm.h" /tmp/wasi-libc/sysroot/include/"bits/msg.h" /tmp/wasi-libc/sysroot/include/"bits/ipc.h" /tmp/wasi-libc/sysroot/include/"bits/ptrace.h" /tmp/wasi-libc/sysroot/include/"bits/statfs.h" /tmp/wasi-libc/sysroot/include/"sys/vfs.h" /tmp/wasi-libc/sysroot/include/"sys/statvfs.h" /tmp/wasi-libc/sysroot/include/"syslog.h" /tmp/wasi-libc/sysroot/include/"sys/syslog.h" /tmp/wasi-libc/sysroot/include/"wait.h" /tmp/wasi-libc/sysroot/include/"sys/wait.h" /tmp/wasi-libc/sysroot/include/"ucontext.h" /tmp/wasi-libc/sysroot/include/"sys/ucontext.h" /tmp/wasi-libc/sysroot/include/"paths.h" /tmp/wasi-libc/sysroot/include/"utmp.h" /tmp/wasi-libc/sysroot/include/"utmpx.h" /tmp/wasi-libc/sysroot/include/"lastlog.h" /tmp/wasi-libc/sysroot/include/"sys/acct.h" /tmp/wasi-libc/sysroot/include/"sys/cachectl.h" /tmp/wasi-libc/sysroot/include/"sys/epoll.h" /tmp/wasi-libc/sysroot/include/"sys/reboot.h" /tmp/wasi-libc/sysroot/include/"sys/swap.h" /tmp/wasi-libc/sysroot/include/"sys/sendfile.h" /tmp/wasi-libc/sysroot/include/"sys/inotify.h" /tmp/wasi-libc/sysroot/include/"sys/quota.h" /tmp/wasi-libc/sysroot/include/"sys/klog.h" /tmp/wasi-libc/sysroot/include/"sys/fsuid.h" /tmp/wasi-libc/sysroot/include/"sys/io.h" /tmp/wasi-libc/sysroot/include/"sys/prctl.h" /tmp/wasi-libc/sysroot/include/"sys/mtio.h" /tmp/wasi-libc/sysroot/include/"sys/mount.h" /tmp/wasi-libc/sysroot/include/"sys/fanotify.h" /tmp/wasi-libc/sysroot/include/"sys/personality.h" /tmp/wasi-libc/sysroot/include/"elf.h" /tmp/wasi-libc/sysroot/include/"link.h" /tmp/wasi-libc/sysroot/include/"bits/link.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi.h" /tmp/wasi-libc/sysroot/include/"scsi/scsi_ioctl.h" /tmp/wasi-libc/sysroot/include/"scsi/sg.h" /tmp/wasi-libc/sysroot/include/"sys/auxv.h" /tmp/wasi-libc/sysroot/include/"pwd.h" /tmp/wasi-libc/sysroot/include/"shadow.h" /tmp/wasi-libc/sysroot/include/"grp.h" /tmp/wasi-libc/sysroot/include/"mntent.h" /tmp/wasi-libc/sysroot/include/"netdb.h" /tmp/wasi-libc/sysroot/include/"resolv.h" /tmp/wasi-libc/sysroot/include/"pty.h" /tmp/wasi-libc/sysroot/include/"dlfcn.h" /tmp/wasi-libc/sysroot/include/"setjmp.h" /tmp/wasi-libc/sysroot/include/"ulimit.h" /tmp/wasi-libc/sysroot/include/"sys/xattr.h" /tmp/wasi-libc/sysroot/include/"wordexp.h" /tmp/wasi-libc/sysroot/include/"spawn.h" /tmp/wasi-libc/sysroot/include/"sys/membarrier.h" /tmp/wasi-libc/sysroot/include/"sys/signalfd.h" /tmp/wasi-libc/sysroot/include/"termios.h" /tmp/wasi-libc/sysroot/include/"sys/termios.h" /tmp/wasi-libc/sysroot/include/"bits/termios.h" /tmp/wasi-libc/sysroot/include/"net/if.h" /tmp/wasi-libc/sysroot/include/"net/if_arp.h" /tmp/wasi-libc/sysroot/include/"net/ethernet.h" /tmp/wasi-libc/sysroot/include/"net/route.h" /tmp/wasi-libc/sysroot/include/"netinet/if_ether.h" /tmp/wasi-libc/sysroot/include/"netinet/ether.h" /tmp/wasi-libc/sysroot/include/"sys/timerfd.h" /tmp/wasi-libc/sysroot/include/"libintl.h" /tmp/wasi-libc/sysroot/include/"sys/sysmacros.h" /tmp/wasi-libc/sysroot/include/"aio.h" /tmp/wasi-libc/sysroot/include/"pthread.h"
#
# Build the startup files.
#
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/dlmalloc/include -MD -MP -o /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o -c /tmp/wasi-libc/dlmalloc/src/dlmalloc.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.c
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.c
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
cd "/tmp/wasi-libc/build" && \
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -c /tmp/wasi-libc/libc-bottom-half/crt/crt1.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-reactor.c /tmp/wasi-libc/libc-bottom-half/crt/crt1-command.c -MD -MP && \
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
mv *.o "/tmp/wasi-libc/sysroot/lib/wasm32-wasi"
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.c
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.c
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: single: No such file or directory
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:399: /tmp/wasi-libc/build/dlmalloc/src/dlmalloc.o] Error 1
make: *** Waiting for unfinished jobs....
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/fcntl.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fadvise.o] Error 1
make: *** [Makefile:399: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/posix_fallocate.o] Error 1
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.c
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.c
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/stdio/renameat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/preadv.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/fcntl/openat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: single: No such file or directory
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/writev.o] Error 1
gcc-8 -O2 -DNDEBUG --target=wasm32-wasi -fno-trapping-math -Wall -Wextra -Werror -Wno-null-pointer-arithmetic -Wno-unused-parameter -Wno-sign-compare -Wno-unused-variable -Wno-unused-function -Wno-ignored-attributes -Wno-missing-braces -Wno-ignored-pragmas -Wno-unused-but-set-variable -Wno-unknown-warning-option -mthread-model single -isystem "/tmp/wasi-libc/sysroot/include" -I/tmp/wasi-libc/libc-bottom-half/headers/private -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src/include -I/tmp/wasi-libc/libc-bottom-half/cloudlibc/src -MD -MP -o /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.o -c /tmp/wasi-libc/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.c
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: single: No such file or directory
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:457: startup_files] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/readv.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstatat.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/uio/pwritev.o] Error 1
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '--target=wasm32-wasi'
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/futimens.o] Error 1
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
gcc-8: error: unrecognized command line option '-mthread-model'; did you mean '-fthread-jumps'?
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/fstat.o] Error 1
make: *** [Makefile:400: /tmp/wasi-libc/build/libc-bottom-half/cloudlibc/src/libc/sys/stat/utimensat.o] Error 1
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
