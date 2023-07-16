
D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsa-b-a-linker-guarda-b-a-linker-guard/b: No such file or directory

could not find native static library `foo`, perhaps an -L flag is missing?

D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\run-make-fulldeps\arguments-non-c-like-enum\arguments-non-c-like-enum/libnonclike.a: No such file or directory

error: linking with `x86_64-w64-mingw32-gcc` failed: exit code: 1
  |
  = note: "x86_64-w64-mingw32-gcc" "-Wl,C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcgZbYmn\\list.def" "-fno-use-linker-plugin" "-Wl,--dynamicbase" "-Wl,--disable-auto-image-base" "-m64" "-Wl,--high-entropy-va" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsc-dynamic-dylibc-dynamic-dylib\\foo.foo.172d8985-cgu.0.rcgu.o" "D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsc-dynamic-dylibc-dynamic-dylib\\foo.21ett6z3hgvj0xmp.rcgu.rmeta" "-L" "D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsc-dynamic-dylibc-dynamic-dylib" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lcfoo" "-Wl,--start-group" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-lstd-6a8d029579976682" "-Wl,--end-group" "-Wl,-Bstatic" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcompiler_builtins-94cb478f7048ff38.rlib" "-Wl,-Bdynamic" "-lkernel32" "-lws2_32" "-lbcrypt" "-ladvapi32" "-luserenv" "-lkernel32" "-lgcc_s" "-lmsvcrt" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-luser32" "-lkernel32" "-Wl,--nxcompat" "-L" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-o" "D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsc-dynamic-dylibc-dynamic-dylib\\foo.dll" "-shared" "-Wl,--out-implib=D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsc-dynamic-dylibc-dynamic-dylib\\libfoo.dll.a" "-nodefaultlibs" "C:\\MORE_SPACE\\x86_64-pc-windows-gnu\\stage2\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
  = note: D:/a/rust/rust/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/6.3.0/../../../../x86_64-w64-mingw32/bin/ld.exe: cannot find -lcfoo
          collect2.exe: error: ld returned 1 exit status

cp: cannot stat 'D:\a\rust\rust\build\x86_64-pc-windows-gnu\test\run-make-fulldeps\extern-multiple-copies\extern-multiple-copies/libfoo1.rlib': No such file or directory

error: error writing dependencies to `D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepsinclude_bytes_depsinclude_bytes_deps\main.d`: The system cannot find the path specified. (os error 3)

/usr/bin/sh: line 1: D:arustrustbuildx86_64-pc-windows-gnutestrun-make-fulldepscompile-stdincompile-stdin/rust_out: No such file or directory
