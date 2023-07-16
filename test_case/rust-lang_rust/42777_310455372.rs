
failures:
---- C:\projects\rust\build\x86_64-pc-windows-msvc\test\error-index.md - Rust_Compiler_Error_Index (line 1285) stdout ----
	error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustdoctest.RWH8fWEAi4XJ\\rust_out.0.o" "/OUT:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustdoctest.RWH8fWEAi4XJ\\rust_out.exe" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "std-c5f066fd9d9c9409.dll.lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-096fad10dd181792.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib"
  = note: rust_out.0.o : error LNK2019: unresolved external symbol printf referenced in function _ZN8rust_out4main17ha22022d8940e3e88E
          C:\Users\appveyor\AppData\Local\Temp\1\rustdoctest.RWH8fWEAi4XJ\rust_out.exe : fatal error LNK1120: 1 unresolved externals
          
error: aborting due to previous error(s)
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:512
thread 'rustc' panicked at 'couldn't compile the test', src\librustdoc\test.rs:278
---- C:\projects\rust\build\x86_64-pc-windows-msvc\test\error-index.md - Rust_Compiler_Error_Index (line 1355) stdout ----
	error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustdoctest.pJgryzUQ8mFi\\rust_out.0.o" "/OUT:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustdoctest.pJgryzUQ8mFi\\rust_out.exe" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "std-c5f066fd9d9c9409.dll.lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-096fad10dd181792.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib"
  = note: rust_out.0.o : error LNK2019: unresolved external symbol printf referenced in function _ZN8rust_out4main17ha22022d8940e3e88E
          C:\Users\appveyor\AppData\Local\Temp\1\rustdoctest.pJgryzUQ8mFi\rust_out.exe : fatal error LNK1120: 1 unresolved externals
          
error: aborting due to previous error(s)
thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:512
thread 'rustc' panicked at 'couldn't compile the test', src\librustdoc\test.rs:278
failures:
    C:\projects\rust\build\x86_64-pc-windows-msvc\test\error-index.md - Rust_Compiler_Error_Index (line 1285)
    C:\projects\rust\build\x86_64-pc-windows-msvc\test\error-index.md - Rust_Compiler_Error_Index (line 1355)
test result: FAILED. 1138 passed; 2 failed; 20 ignored; 0 measured; 0 filtered out
