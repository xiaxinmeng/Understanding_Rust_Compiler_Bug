
[00:55:02] ===============================================================================
[00:55:02] running: "make" "build_lib_static" "-j" "4"
[00:55:02] /android/ndk/arm64-21/bin/aarch64-linux-android-gcc -std=gnu11 -Wall -Werror=declaration-after-statement -Wsign-compare -pipe -g3 -fvisibility=hidden -O3 -funroll-loops -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -c -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -D_GNU_SOURCE -D_REENTRANT -I/checkout/src/liballoc_jemalloc/../jemalloc/include -Iinclude -o src/jemalloc.o /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c
[00:55:02] /android/ndk/arm64-21/bin/aarch64-linux-android-gcc -std=gnu11 -Wall -Werror=declaration-after-statement -Wsign-compare -pipe -g3 -fvisibility=hidden -O3 -funroll-loops -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -c -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -D_GNU_SOURCE -D_REENTRANT -I/checkout/src/liballoc_jemalloc/../jemalloc/include -Iinclude -o src/arena.o /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c
[00:55:02] /android/ndk/arm64-21/bin/aarch64-linux-android-gcc -std=gnu11 -Wall -Werror=declaration-after-statement -Wsign-compare -pipe -g3 -fvisibility=hidden -O3 -funroll-loops -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -c -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -D_GNU_SOURCE -D_REENTRANT -I/checkout/src/liballoc_jemalloc/../jemalloc/include -Iinclude -o src/base.o /checkout/src/liballoc_jemalloc/../jemalloc/src/base.c
[00:55:02] /android/ndk/arm64-21/bin/aarch64-linux-android-gcc -std=gnu11 -Wall -Werror=declaration-after-statement -Wsign-compare -pipe -g3 -fvisibility=hidden -O3 -funroll-loops -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -c -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -D_GNU_SOURCE -D_REENTRANT -I/checkout/src/liballoc_jemalloc/../jemalloc/include -Iinclude -o src/atomic.o /checkout/src/liballoc_jemalloc/../jemalloc/src/atomic.c
[00:55:02] Makefile:293: recipe for target 'src/arena.o' failed
[00:55:02] /android/ndk/arm64-21/bin/aarch64-linux-android-gcc -std=gnu11 -Wall -Werror=declaration-after-statement -Wsign-compare -pipe -g3 -fvisibility=hidden -O3 -funroll-loops -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -c -O2 -ffunction-sections -fdata-sections -fPIC -ffunction-sections -fdata-sections -fPIC -D_GNU_SOURCE -D_REENTRANT -I/checkout/src/liballoc_jemalloc/../jemalloc/include -Iinclude -o src/bitmap.o /checkout/src/liballoc_jemalloc/../jemalloc/src/bitmap.c
[00:55:02] 
[00:55:02] 
[00:55:02] command did not execute successfully: "make" "build_lib_static" "-j" "4"
[00:55:02] expected success, got: exit code: 2
[00:55:02] 
[00:55:02] 
[00:55:02] 
[00:55:02] --- stderr
[00:55:02] In file included from /android/ndk/arm64-21/sysroot/usr/include/sys/syscall.h:36:0,
[00:55:02]                  from /checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/jemalloc_internal_decls.h:13,
[00:55:02]                  from include/jemalloc/internal/jemalloc_internal.h:5,
[00:55:02]                  from /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c:2:
[00:55:02] /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c: In function 'init_thp_initially_huge':
[00:55:02] /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c:3812:20: error: '__NR_open' undeclared (first use in this function)
[00:55:02]   fd = (int)syscall(SYS_open,
[00:55:02]                     ^
[00:55:02] /checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c:3812:20: note: each undeclared identifier is reported only once for each function it appears in
[00:55:02] make: *** [src/arena.o] Error 1
[00:55:02] make: *** Waiting for unfinished jobs....
[00:55:02] 
[00:55:16] error: build failed
[00:55:16] 
[00:55:16] 
[00:55:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "aarch64-linux-android" "--release" "--locked" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
