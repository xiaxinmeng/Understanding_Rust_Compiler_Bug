
Building stage2 tool clippy-driver (armv7-unknown-linux-gnueabihf)
   Compiling backtrace-sys v0.1.24
   Compiling memchr v2.1.1
   Compiling ryu v0.2.7
   Compiling idna v0.1.5
   Compiling smallvec v0.6.7
   Compiling pulldown-cmark v0.2.0
   Compiling clippy v0.0.212 (/checkout/src/tools/clippy)
   Compiling toml v0.4.10
error: failed to run custom build command for `backtrace-sys v0.1.24`
process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/backtrace-sys-0483c43875de1fe5/build-script-build` (exit code: 101)
--- stdout
TARGET = Some("x86_64-unknown-linux-gnu")
OPT_LEVEL = Some("2")
HOST = Some("x86_64-unknown-linux-gnu")
CC_x86_64-unknown-linux-gnu = None
CC_x86_64_unknown_linux_gnu = None
HOST_CC = None
CC = Some("sccache armv7-unknown-linux-gnueabihf-gcc")
CFLAGS_x86_64-unknown-linux-gnu = None
CFLAGS_x86_64_unknown_linux_gnu = None
HOST_CFLAGS = None
CFLAGS = Some("-ffunction-sections -fdata-sections -fPIC -march=armv7-a")
DEBUG = Some("false")
running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-m64" "-I" "src/libbacktrace" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/backtrace-sys-b7b7de2d05114cff/out" "-fvisibility=hidden" "-DBACKTRACE_ELF_SIZE=64" "-DBACKTRACE_SUPPORTED=1" "-DBACKTRACE_USES_MALLOC=1" "-DBACKTRACE_SUPPORTS_THREADS=0" "-DBACKTRACE_SUPPORTS_DATA=0" "-DHAVE_DL_ITERATE_PHDR=1" "-D_GNU_SOURCE=1" "-D_LARGE_FILES=1" "-Dbacktrace_full=__rbt_backtrace_full" "-Dbacktrace_dwarf_add=__rbt_backtrace_dwarf_add" "-Dbacktrace_initialize=__rbt_backtrace_initialize" "-Dbacktrace_pcinfo=__rbt_backtrace_pcinfo" "-Dbacktrace_syminfo=__rbt_backtrace_syminfo" "-Dbacktrace_get_view=__rbt_backtrace_get_view" "-Dbacktrace_release_view=__rbt_backtrace_release_view" "-Dbacktrace_alloc=__rbt_backtrace_alloc" "-Dbacktrace_free=__rbt_backtrace_free" "-Dbacktrace_vector_finish=__rbt_backtrace_vector_finish" "-Dbacktrace_vector_grow=__rbt_backtrace_vector_grow" "-Dbacktrace_vector_release=__rbt_backtrace_vector_release" "-Dbacktrace_close=__rbt_backtrace_close" "-Dbacktrace_open=__rbt_backtrace_open" "-Dbacktrace_print=__rbt_backtrace_print" "-Dbacktrace_simple=__rbt_backtrace_simple" "-Dbacktrace_qsort=__rbt_backtrace_qsort" "-Dbacktrace_create_state=__rbt_backtrace_create_state" "-Dbacktrace_uncompress_zdebug=__rbt_backtrace_uncompress_zdebug" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/backtrace-sys-b7b7de2d05114cff/out/src/libbacktrace/alloc.o" "-c" "src/libbacktrace/alloc.c"
cargo:warning=armv7-unknown-linux-gnueabihf-gcc: error: unrecognized command line option '-m64'
exit code: 1
