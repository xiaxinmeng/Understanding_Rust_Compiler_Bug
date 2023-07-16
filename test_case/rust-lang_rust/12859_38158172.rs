
make[1]: Leaving directory `/home/mvdnes/rust/src/compiler-rt'
cp: i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/libcompiler-rt.a
compile: i686-w64-mingw32/rt/rust_builtin.o
compile: i686-w64-mingw32/rt/rust_android_dummy.o
compile: i686-w64-mingw32/rt/rust_test_helpers.o
compile: i686-w64-mingw32/rt/rust_try.o
compile: i686-w64-mingw32/rt/arch/i386/record_sp.o
link: i686-w64-mingw32/rt/rustrt.lib
touch i686-w64-mingw32/rt/backtrace.lib
oxidize: i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/libstd
error: linking with `i686-w64-mingw32-g++` failed: exit code: 1
note: i686-w64-mingw32-g++ arguments: '-m32' '-L/home/mvdnes/rust/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib' '-o' 'i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std-31b43f22-0.10-pre.dll' 'i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o' '-lmorestack' 'i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.metadata.o' '-shared-libgcc' '-Li686-w64-mingw32/rt' '-L.' '-L/home/mvdnes/rust/.rust' '-L/home/mvdnes/rust' '-lrustrt' '-lgcc_s' '-shared' '-lcompiler-rt'
note: i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0xd14): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x1e26): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x2426): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x2cdd): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x44a2): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x585d): more undefined references to `__Unwind_Resume' follow
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x8f284): undefined reference to `__Unwind_RaiseException'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x8f2fd): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x8f307): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x8f518): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x8f522): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x8f728): undefined reference to `__Unwind_Resume'
i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o:(.text+0x93616): more undefined references to `__Unwind_Resume' follow
/usr/bin/i686-w64-mingw32-ld: i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/std.o: bad reloc address 0x14 in section `.data'
collect2: ld returned 1 exit status

error: aborting due to previous error
make: *** [i686-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/stamp.std] Error 101
