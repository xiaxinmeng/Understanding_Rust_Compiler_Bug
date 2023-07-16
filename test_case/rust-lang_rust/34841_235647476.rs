
cc1: warning: command line option '-ftarget=mips-linux-gnu' is valid for Java but not for C
In file included from /usr/mips-linux-gnu/include/features.h:391:0,
                 from /usr/mips-linux-gnu/include/limits.h:25,
                 from /usr/lib/gcc-cross/mips-linux-gnu/5/include-fixed/limits.h:168,
                 from /usr/lib/gcc-cross/mips-linux-gnu/5/include-fixed/syslimits.h:7,
                 from /usr/lib/gcc-cross/mips-linux-gnu/5/include-fixed/limits.h:34,
                 from /buildslave/rust-buildbot/slave/beta-dist-rustc-cross-linux/build/src/compiler-rt/lib/builtins/int_lib.h:67,
                 from /buildslave/rust-buildbot/slave/beta-dist-rustc-cross-linux/build/src/compiler-rt/lib/builtins/absvdi2.c:15:
/usr/mips-linux-gnu/include/gnu/stubs.h:8:33: fatal error: gnu/stubs-o32_soft.h: No such file or directory
compilation terminated.
make[4]: *** [lib/builtins/CMakeFiles/clang_rt.builtins-mips.dir/absvdi2.c.o] Error 1
make[4]: Leaving directory `/buildslave/rust-buildbot/slave/beta-dist-rustc-cross-linux/build/obj/mips-unknown-linux-gnu/rt/compiler-rt'
make[3]: *** [lib/builtins/CMakeFiles/clang_rt.builtins-mips.dir/all] Error 2
make[3]: Leaving directory `/buildslave/rust-buildbot/slave/beta-dist-rustc-cross-linux/build/obj/mips-unknown-linux-gnu/rt/compiler-rt'
make[2]: *** [lib/builtins/CMakeFiles/clang_rt.builtins-mips.dir/rule] Error 2
make[1]: *** [clang_rt.builtins-mips] Error 2make[2]: Leaving directory `/buildslave/rust-buildbot/slave/beta-dist-rustc-cross-linux/build/obj/mips-unknown-linux-gnu/rt/compiler-rt'
make[1]: Leaving directory `/buildslave/rust-buildbot/slave/beta-dist-rustc-cross-linux/build/obj/mips-unknown-linux-gnu/rt/compiler-rt'

make: *** [mips-unknown-linux-gnu/rt/libcompiler-rt.a] Error 2
