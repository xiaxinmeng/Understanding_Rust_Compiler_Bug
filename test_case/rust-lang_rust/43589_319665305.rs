
   Compiling term v0.0.0 (file:///C:/projects/rust/src/libterm)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\deps\\term-c6d433459a54f839.0.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\deps\\term-c6d433459a54f839.dll" "/DEF:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.eYw8sZPlb2Lc\\lib.def" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\deps\\term-c6d433459a54f839.crate.metadata.o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\deps\\term-c6d433459a54f839.crate.allocator.o" "/OPT:REF,ICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\release\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "kernel32.lib" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "std-6894941a948c9efd.dll.lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.eYw8sZPlb2Lc\\libcompiler_builtins-dd1b2e32329767b3.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "libcmt.lib" "/DLL" "/IMPLIB:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0-test\\x86_64-pc-windows-msvc\\release\\deps\\term-c6d433459a54f839.dll.lib"
  = note:    Creating library C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-test\x86_64-pc-windows-msvc\release\deps\term-c6d433459a54f839.dll.lib and object C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-test\x86_64-pc-windows-msvc\release\deps\term-c6d433459a54f839.dll.exp
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_alloc referenced in function __rust_alloc
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_oom referenced in function __rust_oom
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_dealloc referenced in function __rust_dealloc
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_usable_size referenced in function __rust_usable_size
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_realloc referenced in function __rust_realloc
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_alloc_zeroed referenced in function __rust_alloc_zeroed
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_alloc_excess referenced in function __rust_alloc_excess
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_realloc_excess referenced in function __rust_realloc_excess
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_grow_in_place referenced in function __rust_grow_in_place
          term-c6d433459a54f839.crate.allocator.o : error LNK2019: unresolved external symbol __rg_shrink_in_place referenced in function __rust_shrink_in_place
          C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-test\x86_64-pc-windows-msvc\release\deps\term-c6d433459a54f839.dll : fatal error LNK1120: 10 unresolved externals
          
error: aborting due to previous error
error: Could not compile `term`.
