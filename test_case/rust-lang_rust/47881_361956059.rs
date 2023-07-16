
error: linking with `link.exe` failed: exit code: 1561
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/tmp/clippy_test_build_base\\enum-glob-import-crate.enum_glob_import_crate0.rcgu.o" "/tmp/clippy_test_build_base\\enum-glob-import-crate.enum_glob_import_crate1.rcgu.o" "/tmp/clippy_test_build_base\\enum-glob-import-crate.enum_glob_import_crate2.rcgu.o" "/OUT:/tmp/clippy_test_build_base\\enum-glob-import-crate.stage-id.exe" "/tmp/clippy_test_build_base\\enum-glob-import-crate.crate.allocator.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:/tmp/clippy_test_build_base" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\release" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\release/deps" "/LIBPATH:/tmp/clippy_test_build_base\\enum-glob-import-crate.stage-id.aux" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "std-8c525ca5930ac7cd.dll.lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-3d7fcf367f7cd6b3.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib"
  = note: LINK : warning LNK4044: unrecognized option '/tmp/clippy_test_build_base\enum-glob-import-crate.enum_glob_import_crate0.rcgu.o'; ignored
          LINK : warning LNK4044: unrecognized option '/tmp/clippy_test_build_base\enum-glob-import-crate.enum_glob_import_crate1.rcgu.o'; ignored
          LINK : warning LNK4044: unrecognized option '/tmp/clippy_test_build_base\enum-glob-import-crate.enum_glob_import_crate2.rcgu.o'; ignored
          LINK : warning LNK4044: unrecognized option '/tmp/clippy_test_build_base\enum-glob-import-crate.crate.allocator.rcgu.o'; ignored
          LINK : warning LNK4001: no object files specified; libraries used
          LINK : warning LNK4068: /MACHINE not specified; defaulting to X64
          LINK : fatal error LNK1561: entry point must be defined
          
