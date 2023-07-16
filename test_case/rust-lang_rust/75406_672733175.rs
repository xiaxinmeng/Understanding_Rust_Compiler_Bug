plain
   Compiling gimli v0.22.0
   Compiling miniz_oxide v0.4.0
   Compiling object v0.20.0
   Compiling addr2line v0.13.0
error: linking with `i686-w64-mingw32-gcc` failed: exit code: 1
  |
  = note: "i686-w64-mingw32-gcc" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-Wl,--dynamicbase" "-Wl,--high-entropy-va" "-Wl,--disable-auto-image-base" "-Wl,--large-address-aware" "-nostartfiles" "D:\\a\\rust\\rust\\mingw32\\i686-w64-mingw32\\lib\\dllcrt2.o" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\rsbegin.o" "-L" "D:\\a\\rust\\rust\\mingw32\\i686-w64-mingw32\\lib" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\self-contained" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\std-001c7c07237f5a53.std.2oi95g00-cgu.0.rcgu.o" "-o" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\std-001c7c07237f5a53.dll" "-Wl,C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\list.def" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\std-001c7c07237f5a53.2hraozlwyo1yhfwv.rcgu.o" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\std-001c7c07237f5a53.1hjeje4fj01jxwde.rcgu.o" "-shared" "-Wl,--out-implib,D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\libstd-001c7c07237f5a53.dll.a" "-nodefaultlibs" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\release\\deps" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\build\\compiler_builtins-9c3d08aa6513a296\\out" "-L" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-ladvapi32" "-lws2_32" "-luserenv" "-Wl,-Bstatic" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libpanic_unwind-b835dd322c68c1f1.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libobject-39cb832dd7b4fe6c.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libaddr2line-4c97ef5a0e76d72a.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libgimli-2fe4a88c7896e835.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\librustc_demangle-64d2fa27da74b60c.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libhashbrown-a5eaa47a1565eb3e.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\librustc_std_workspace_alloc-62f28998d5602550.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libunwind-1cae5105b9d9a6cf.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libcfg_if-a8c586b295602734.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\liblibc-ecd3b8c387994089.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\liballoc-cab34448e6c5a1a9.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\librustc_std_workspace_core-b320f0b9b23c3ba1.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libcore-bd38834d04123d9c.rlib" "-Wl,--no-whole-archive" "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustchXSkpj\\libcompiler_builtins-4ad5645d782c03f2.rlib" "-Wl,-Bdynamic" "-lmsvcrt" "-lmingwex" "-lmingw32" "-lmsvcrt" "-luser32" "-lkernel32" "-lgcc_s" "-lgcc" "-lkernel32" "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\rsend.o"
  = note: D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/ld.exe: unrecognized option '--high-entropy-va'

          D:/a/rust/rust/mingw32/bin/../lib/gcc/i686-w64-mingw32/6.3.0/../../../../i686-w64-mingw32/bin/ld.exe: use the --help option for usage information

          collect2.exe: error: ld returned 1 exit status
          

error: aborting due to previous error


error: could not compile `std`.

To learn more, run the command again with --verbose.
command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/test/compile-fail
Build completed unsuccessfully in 0:16:31
Build completed unsuccessfully in 0:16:31
make: *** [Makefile:82: ci-mingw-subset-1] Error 1
  local time: Wed Aug 12 08:32:58 CUT 2020
  network time: Wed, 12 Aug 2020 08:32:58 GMT
== end clock drift check ==
== end clock drift check ==
##[error]Process completed with exit code 2.
Terminate orphan process: pid (4700) (python)
Terminate orphan process: pid (1464) (sccache)
