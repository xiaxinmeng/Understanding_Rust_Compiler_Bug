plain
[RUSTC-TIMING] unicode_width test:false 0.118
[RUSTC-TIMING] getopts test:false 3.938
[RUSTC-TIMING] proc_macro test:false 18.904
   Compiling test v0.0.0 (D:\a\rust\rust\library\test)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\x64\\link.exe" "/DEF:C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustckTa4vS\\lib.def" "/NOLOGO" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-6464307edd00076d.test.00e0ddf2-cgu.0.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-6464307edd00076d.3mcoy5avu9ef5mmi.rcgu.rmeta" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\release\\deps" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-d23c84560eca12a0\\out" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "kernel32.lib" "/WHOLEARCHIVE:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\libgetopts-04f14876bdd48cb7.rlib" "/WHOLEARCHIVE:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_width-c74fe7d359ba4245.rlib" "/WHOLEARCHIVE:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\librustc_std_workspace_std-ac92d3dd0da3e4c2.rlib" "/LIBPATH:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps" "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\libcompiler_builtins-033009b95f350faf.rlib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "advapi32.lib" "userenv.lib" "kernel32.lib" "libcmt.lib" "/NXCOMPAT" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-6464307edd00076d.dll" "/OPT:REF,ICF" "/DLL" "/IMPLIB:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-6464307edd00076d.dll.lib" "/DEBUG"
  = note: lib.def : error LNK2001: unresolved external symbol __rust_alloc

          lib.def : error LNK2001: unresolved external symbol __rust_alloc_zeroed

          lib.def : error LNK2001: unresolved external symbol __rust_dealloc

          lib.def : error LNK2001: unresolved external symbol __rust_realloc

          D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\test-6464307edd00076d.dll.lib : fatal error LNK1120: 4 unresolved externals
          

[RUSTC-TIMING] test test:false 20.307
error: could not compile `test` due to previous error
