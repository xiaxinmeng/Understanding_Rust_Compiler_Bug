plain
[00:56:35]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:56:35]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:56:35]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:56:35]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:56:46] error: failed to run custom build command for `alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)`
[00:56:46] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/alloc_jemalloc-630168dfefeaf0fd/build-script-build` (exit code: 1)
[00:56:46] --- stdout
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/scripts/gen_travis.py
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/ticker.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/spin.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/ckh.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/prng.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/hash.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/nstime.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/chunk.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/chunk_mmap.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/stats.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/extent.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/zone.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/prof.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/bitmap.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/util.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/arena.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/rtree.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/ctl.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/atomic.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/huge.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/quarantine.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/tsd.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/witness.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/chunk_dss.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/pages.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/tcache.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/mutex.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/mb.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/base.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/src/valgrind.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/build-aux/install-sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/build-aux/config.sub
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/build-aux/config.guess
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/config.stamp.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/INSTALL
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/README
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/configure.ac
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/configure
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/.autom4te.cfg
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/ticker.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_active.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/junk_alloc.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/smoothstep.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/ckh.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/arena_reset.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/extent_quantize.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/junk_alloc.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prng.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/hash.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/junk_free.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/mq.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/nstime.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/math.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_accum.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/stats.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/pack.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/a0.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/lg_chunk.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/run_quantize.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/bitmap.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/size_classes.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/decay.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/util.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/junk_free.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_accum.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/rb.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_reset.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/zero.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/rtree.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_reset.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_tctx.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/quarantine.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_idump.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/atomic.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/quarantine.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/decay.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/junk.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/tsd.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/zero.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/pack.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/witness.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/stats_print.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/ph.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_gdump.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/lg_chunk.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/arena_reset.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/junk.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_active.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_thread_name.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/pages.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/ql.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/SFMT.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/qr.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/fork.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_idump.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_gdump.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/prof_thread_name.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/mtx.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/unit/mallctl.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/btalloc.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/btalloc_1.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/mq.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/math.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/test.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/btalloc_0.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/SFMT.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/thd.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/mtx.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/src/timer.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/stress/microbench.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/test.sh.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/thread_arena.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/xallocx.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/aligned_alloc.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/mallocx.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/xallocx.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/chunk.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/mallocx.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/overflow.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/chunk.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/rallocx.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/sdallocx.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/allocated.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/posix_memalign.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/MALLOCX_ARENA.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/integration/thread_tcache_enabled.c
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/test.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/thd.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params11213.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/mq.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/jemalloc_test_defs.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params2281.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params86243.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params132049.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params19937.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-alti.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/mtx.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params44497.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/math.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-sse2.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params4253.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params607.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params1279.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/btalloc.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/timer.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/SFMT-params216091.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/test/include/test/jemalloc_test.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/.travis.yml
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/.gitignore
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/Makefile.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/ChangeLog
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/bin/jemalloc.sh.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/bin/jeprof.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/bin/jemalloc-config.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/autogen.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/jemalloc/jemalloc.vcxproj
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/jemalloc/jemalloc.vcxproj.filters
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/test_threads/test_threads.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/test_threads/test_threads.vcxproj.filters
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/test_threads/test_threads.vcxproj
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/test_threads/test_threads_main.cpp
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/projects/vc2015/test_threads/test_threads.cpp
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/jemalloc_vc2015.sln
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/msvc/ReadMe.txt
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/COPYING
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/.appveyor.yml
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/jemalloc.pc.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/msvc_compat/strings.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/msvc_compat/C99/stdint.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/msvc_compat/C99/stdbool.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/msvc_compat/windows_extra.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc_macros.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc_rename.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/arena.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/chunk_mmap.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/prof.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/private_unnamespace.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/qr.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/huge.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/smoothstep.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/ql.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/mutex.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/size_classes.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/spin.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/valgrind.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/assert.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/ph.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/public_namespace.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/atomic.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/tsd.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/bitmap.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/hash.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/jemalloc_internal_decls.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/base.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/stats.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/ctl.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/chunk.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/extent.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/chunk_dss.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/pages.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/ckh.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/jemalloc_internal_macros.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/rb.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/witness.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/jemalloc_internal.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/jemalloc_internal_defs.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/public_unnamespace.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/util.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/prng.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/ticker.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/nstime.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/quarantine.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/tcache.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/rtree.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/smoothstep.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/mb.h
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/private_namespace.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/internal/private_symbols.txt
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc_protos.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc_mangle.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc_defs.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/include/jemalloc/jemalloc_typedefs.h.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/coverage.sh
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/.gitattributes
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/doc/html.xsl.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/doc/jemalloc.xml.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/doc/manpages.xsl.in
[00:56:46] cargo:rerun-if-changed=/checkout/src/liballoc_jemalloc/../jemalloc/doc/stylesheet.xsl
[00:56:46] cargo:rustc-link-lib=static=jemalloc_pic
[00:56:46] cargo:rustc-link-search=native=/checkout/obj/build/arm-unknown-linux-musleabi/native/jemalloc/lib
[00:56:46] running: "sh" "/checkout/src/liballoc_jemalloc/../jemalloc/configure" "--with-jemalloc-prefix=je_" "--host=arm-unknown-linux-musleabi" "--build=x86_64-unknown-linux-gnu"
[00:56:46] checking for xsltproc... false
[00:56:46] checking for arm-unknown-linux-musleabi-gcc... sccache /musl-arm/bin/musl-gcc
[00:56:46] checking for C compiler default output file name... a.out
[00:56:46] checking for suffix of executables... 
[00:56:46] checking whether we are cross compiling... yes
[00:56:46] checking for suffix of object files... o
[00:56:46] checking for suffix of object files... o
[00:56:46] checking whether we are using the GNU C compiler... yes
[00:56:46] checking whether sccache /musl-arm/bin/musl-gcc accepts -g... yes
[00:56:46] checking for sccache /musl-arm/bin/musl-gcc option to accept ISO C89... none needed
[00:56:46] checking whether compiler is cray... no
[00:56:46] checking whether compiler supports -std=gnu11... yes
[00:56:46] checking whether compiler supports -Wall... yes
[00:56:46] checking whether compiler supports -Werror=declaration-after-statement... yes
[00:56:46] checking whether compiler supports -Wshorten-64-to-32... no
[00:56:46] checking whether compiler supports -Wsign-compare... yes
[00:56:46] checking whether compiler supports -g3... yes
[00:56:46] checking whether compiler supports -g3... yes
[00:56:46] checking how to run the C preprocessor... sccache /musl-arm/bin/musl-gcc -E
[00:56:46] checking for egrep... /bin/grep -E
[00:56:46] checking for ANSI C header files... yes
[00:56:46] checking for sys/types.h... yes
[00:56:46] checking for sys/stat.h... yes
[00:56:46] checking for sys/stat.h... yes
[00:56:46] checking for stdlib.h... yes
[00:56:46] checking for string.h... yes
[00:56:46] checking for memory.h... yes
[00:56:46] checking for strings.h... yes
[00:56:46] checking for inttypes.h... yes
[00:56:46] checking for stdint.h... yes
[00:56:46] checking for unistd.h... yes
[00:56:46] checking whether byte ordering is bigendian... no
[00:56:46] checking size of void *... 4
[00:56:46] checking size of int... 4
[00:56:46] checking size of long... 4
[00:56:46] checking size of long long... 8
[00:56:46] checking size of intmax_t... 8
[00:56:46] checking host system type... arm-unknown-linux-musleabi
[00:56:46] checking for arm-unknown-linux-musleabi-ar... ar
[00:56:46] checking malloc.h usability... yes
[00:56:46] checking malloc.h presence... yes
[00:56:46] checking malloc.h presence... yes
[00:56:46] checking for malloc.h... yes
[00:56:46] checking whether malloc_usable_size definition can use const argument... no
[00:56:46] checking for library containing log... none required
[00:56:46] checking whether __attribute__ syntax is compilable... yes
[00:56:46] checking whether compiler supports -fvisibility=hidden... yes
[00:56:46] checking whether compiler supports -Werror... yes
[00:56:46] checking whether compiler supports -herror_on_warning... yes
[00:56:46] checking whether tls_model attribute is compilable... yes
[00:56:46] checking whether compiler supports -Werror... yes
[00:56:46] checking whether compiler supports -herror_on_warning... yes
[00:56:46] checking whether alloc_size attribute is compilable... yes
[00:56:46] checking whether compiler supports -Werror... yes
[00:56:46] checking whether compiler supports -herror_on_warning... yes
[00:56:46] checking whether format(gnu_printf, ...) attribute is compilable... yes
[00:56:46] checking whether compiler supports -Werror... yes
[00:56:46] checking whether compiler supports -herror_on_warning... yes
[00:56:46] checking whether format(printf, ...) attribute is compilable... yes
[00:56:46] checking for arm-unknown-linux-musleabi-ranlib... ar s
[00:56:46] checking for ld... /usr/bin/ld
[00:56:46] checking for autoconf... false
[00:56:46] checking for memalign... yes
[00:56:46] checking for memalign... yes
[00:56:46] checking for valloc... yes
[00:56:46] checking whether compiler supports -O3... yes
[00:56:46] checking whether compiler supports -funroll-loops... yes
[00:56:46] checking configured backtracing method... N/A
[00:56:46] checking for sbrk... yes
[00:56:46] checking whether utrace(2) is compilable... no
[00:56:46] checking whether valgrind is compilable... no
[00:56:46] checking whether a program using __builtin_unreachable is compilable... yes
[00:56:46] checking whether a program using __builtin_ffsl is compilable... no
[00:56:46] checking whether a program using ffsl is compilable... no
[00:56:46] 
[00:56:46] 
[00:56:46] command did not execute successfully: "sh" "/checkout/src/liballoc_jemalloc/../jemalloc/configure" "--with-jemalloc-prefix=je_" "--host=arm-unknown-linux-musleabi" "--build=x86_64-unknown-linux-gnu"
[00:56:46] 
[00:56:46] 
[00:56:46] 
[00:56:46] --- stderr
[00:56:46] --- stderr
[00:56:46] configure: error: Cannot build without ffsl(3) or __builtin_ffsl()
[00:56:46] warning: build failed, waiting for other jobs to finish...
[00:57:08] [RUSTC-TIMING] core test:false 33.322
[00:57:08] error: build failed
[00:57:08] error: build failed
[00:57:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-unknown-linux-musleabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:57:08] expected success, got: exit code: 101
[00:57:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:57:08] travis_fold:end:stage2-std

[00:57:08] travis_time:end:stage2-std:start=1524202339411697424,finish=1524202373192482940,duration=33780785516

