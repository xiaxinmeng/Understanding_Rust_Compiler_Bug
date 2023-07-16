
PATH=/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib:$PATH   x86_64-unknown-linux-gnu/stage2/bin/rustc --cfg stage2  -O --cfg rtopt --cfg debug -C rpath  --target=i686-w64-mingw32 -C linker=i686-w64-mingw32-g++ -C ar=i686-w64-mingw32-ar   -D warnings -L "i686-w64-mingw32/rt" -L "" -L ""  --out-dir x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib -C extra-filename=-4e7c5e5c /home/cmr/src/rust3/src/librustrt/lib.rs
error: linking with `i686-w64-mingw32-g++` failed: exit code: 1
note: i686-w64-mingw32-g++ '-m32' '-L' '/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib' '-o' 'x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt-4e7c5e5c.dll' 'x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o' '-Wl,--whole-archive' '-lmorestack' '-Wl,--no-whole-archive' 'x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.metadata.o' '-shared-libgcc' '-Wl,--enable-long-section-names' '/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/libcollections-4e7c5e5c.rlib' '/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/libunicode-4e7c5e5c.rlib' '/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/liballoc-4e7c5e5c.rlib' '/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/liblibc-4e7c5e5c.rlib' '/home/cmr/src/rust3/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/libcore-4e7c5e5c.rlib' '-L' 'i686-w64-mingw32/rt' '-L' '.' '-L' '/home/cmr/.rust/lib' '-L' '/home/cmr/src/rust3/build/.rust' '-L' '/home/cmr/src/rust3/build' '-L' '/home/cmr/.rust' '-Wl,-Bdynamic' '-lgcc_s' '-Wl,-Bstatic' '-lrustrt_native' '-Wl,-Bdynamic' '-shared' '-lcompiler-rt'
note: x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x31): undefined reference to `__gcc_personality_v0'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0xcb5): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0xcbd): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0xde6): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0xeb7): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x1a0b): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x1eec): more undefined references to `_Unwind_Resume' follow
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x6dd3): undefined reference to `_Unwind_RaiseException'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x6e8f): undefined reference to `__gcc_personality_v0'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x70de): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x7578): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x7a4f): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x7b04): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x7dbf): undefined reference to `_Unwind_Resume'
x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o:(.text+0x7e37): more undefined references to `_Unwind_Resume' follow
/usr/lib/gcc/i686-w64-mingw32/4.9.0/../../../../i686-w64-mingw32/bin/ld: x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/rustrt.o: bad reloc address 0x4 in section `.data'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
/home/cmr/src/rust3/mk/target.mk:166: recipe for target 'x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/stamp.rustrt' failed
make: *** [x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-w64-mingw32/lib/stamp.rustrt] Error 101
