plain
[RUSTC-TIMING] proc_macro test:false 16.284
   Compiling test v0.0.0 (D:\a\rust\rust\library\test)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\x64\\link.exe" "/DEF:C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcLNXhpH\\lib.def" "/NOLOGO" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-f79f3bb6c960cb74.test.0b6420be-cgu.0.rcgu.o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-f79f3bb6c960cb74.4d0fzqaaw94cf2l0.rcgu.rmeta" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\release\\deps" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-c19e230456be9b68\\out" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "kernel32.lib" "/WHOLEARCHIVE:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\libgetopts-75c63eec14993635.rlib" "/WHOLEARCHIVE:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\libunicode_width-5a8e1d9586e5c6cc.rlib" "/WHOLEARCHIVE:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\librustc_std_workspace_std-f33d5479e7886b2b.rlib" "/LIBPATH:\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps" "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\libcompiler_builtins-82029df522cc1015.rlib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "advapi32.lib" "userenv.lib" "kernel32.lib" "libcmt.lib" "/NXCOMPAT" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-f79f3bb6c960cb74.dll" "/OPT:REF,ICF" "/DLL" "/IMPLIB:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\x86_64-pc-windows-msvc\\release\\deps\\test-f79f3bb6c960cb74.dll.lib" "/DEBUG"
  = note: lib.def : error LNK2001: unresolved external symbol __rust_alloc

          lib.def : error LNK2001: unresolved external symbol __rust_alloc_zeroed

          lib.def : error LNK2001: unresolved external symbol __rust_dealloc

          lib.def : error LNK2001: unresolved external symbol __rust_realloc

          D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps\test-f79f3bb6c960cb74.dll.lib : fatal error LNK1120: 4 unresolved externals
          

[RUSTC-TIMING] test test:false 18.862
error: could not compile `test` due to previous error
