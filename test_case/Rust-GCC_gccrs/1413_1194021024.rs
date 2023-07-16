
libtool: compile:  /mnt/c/Users/xxx/xxx/CrossDevelopment/rust/gccrs-build2/./gcc/xgcc -B/mnt/c/Users/xxx/xxx/CrossDevelopment/rust/gccrs-build2/./gcc/ -B/mnt/c/Users/xxx/xxx/CrossDevelopment/rust/gccrs-cross/sh4-elf/bin/ -B/mnt/c/Users/xxx/xxx/CrossDevelopment/rust/gccrs-cross/sh4-elf/lib/ -isystem /mnt/c/Users/xxx/xxx/CrossDevelopment/rust/gccrs-cross/sh4-elf/include -isystem /mnt/c/Users/xxx/xxx/CrossDevelopment/rust/gccrs-cross/sh4-elf/sys-include -DHAVE_CONFIG_H -I. -I../../../gccrs/libssp -Wall -g -O2 -MT ssp.lo -MD -MP -MF .deps/ssp.Tpo -c ../../../gccrs/libssp/ssp.c -o ssp.o
../../../gccrs/libssp/ssp.c: In function '__guard_setup':
../../../gccrs/libssp/ssp.c:93:12: warning: implicit declaration of function 'open' [-Wimplicit-function-declaration]
   93 |   int fd = open ("/dev/urandom", O_RDONLY);
      |            ^~~~
../../../gccrs/libssp/ssp.c:93:34: error: 'O_RDONLY' undeclared (first use in this function)
   93 |   int fd = open ("/dev/urandom", O_RDONLY);
      |                                  ^~~~~~~~
../../../gccrs/libssp/ssp.c:93:34: note: each undeclared identifier is reported only once for each function it appears in
../../../gccrs/libssp/ssp.c:96:7: error: unknown type name 'ssize_t'
   96 |       ssize_t size = read (fd, &__stack_chk_guard,
      |       ^~~~~~~
../../../gccrs/libssp/ssp.c:96:22: warning: implicit declaration of function 'read' [-Wimplicit-function-declaration]
   96 |       ssize_t size = read (fd, &__stack_chk_guard,
      |                      ^~~~
../../../gccrs/libssp/ssp.c:98:7: warning: implicit declaration of function 'close' [-Wimplicit-function-declaration]
   98 |       close (fd);
      |       ^~~~~
../../../gccrs/libssp/ssp.c: At top level:
../../../gccrs/libssp/ssp.c:113:25: error: unknown type name 'size_t'
  113 | fail (const char *msg1, size_t msg1len, const char *msg3)
      |                         ^~~~~~
../../../gccrs/libssp/ssp.c:36:1: note: 'size_t' is defined in header '<stddef.h>'; did you forget to '#include <stddef.h>'?
   35 | #include "config.h"
  +++ |+#include <stddef.h>
   36 | #ifdef HAVE_ALLOCA_H
