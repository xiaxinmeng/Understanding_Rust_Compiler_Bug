
C:\Users\cyber\Rust\rust>rustup default nightly-2018-04-06
info: using existing install for 'nightly-2018-04-06-x86_64-pc-windows-msvc'
info: default toolchain set to 'nightly-2018-04-06-x86_64-pc-windows-msvc'

  nightly-2018-04-06-x86_64-pc-windows-msvc unchanged - rustc 1.27.0-nightly (48fa6f963 2018-04-05)


C:\Users\cyber\Rust\rust>x.py clean && x.py build
Updating only changed submodules
Submodules updated in 0.12 seconds
   Compiling unicode-xid v0.1.0
   Compiling winapi v0.3.4
   Compiling dtoa v0.4.2
   Compiling ordermap v0.3.5
   Compiling serde v1.0.37
   Compiling cfg-if v0.1.2
   Compiling cc v1.0.9
   Compiling libc v0.2.40
   Compiling fixedbitset v0.1.9
   Compiling num-traits v0.2.2
   Compiling itoa v0.4.1
   Compiling getopts v0.2.17
   Compiling build_helper v0.1.0 (file:///C:/Users/cyber/Rust/rust/src/build_helper)
   Compiling lazy_static v0.2.11
   Compiling filetime v0.1.15
   Compiling proc-macro2 v0.3.1
   Compiling num_cpus v1.8.0
   Compiling petgraph v0.4.12
   Compiling quote v0.5.1
   Compiling cmake v0.1.29
   Compiling syn v0.13.1
   Compiling time v0.1.39
   Compiling serde_derive_internals v0.23.0
   Compiling serde_json v1.0.13
   Compiling toml v0.4.5
   Compiling serde_derive v1.0.37
   Compiling bootstrap v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 34.87 secs
Build completed successfully in 0:00:36
Updating only changed submodules
Submodules updated in 0.12 seconds
    Finished dev [unoptimized] target(s) in 0.0 secs
Building stage0 std artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
   Compiling cc v1.0.9
   Compiling core v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libcore)
   Compiling unwind v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libunwind)
   Compiling build_helper v0.1.0 (file:///C:/Users/cyber/Rust/rust/src/build_helper)
   Compiling std v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libstd)
   Compiling compiler_builtins v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/rustc/compiler_builtins_shim)
   Compiling libc v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/rustc/libc_shim)
   Compiling alloc v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/liballoc)
   Compiling std_unicode v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libstd_unicode)
   Compiling panic_abort v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libpanic_abort)
   Compiling alloc_system v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/liballoc_system)
   Compiling panic_unwind v0.0.0 (file:///C:/Users/cyber/Rust/rust/src/libpanic_unwind)
error: linking with `C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.13.26128\bin\HostX64\x64\link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\VC\\Tools\\MSVC\\14.13.26128\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std0-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std1-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std10-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std11-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std12-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std13-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std14-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std15-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std2-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std3-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std4-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std5-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std6-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std7-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std8-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.std9-6cc1ac623fafb9d4e8d09970e15b06a.rs.rcgu.o" "/OUT:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.dll" "/DEF:C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\lib.def" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.crate.metadata.rcgu.o" "C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.crate.allocator.rcgu.o" "/OPT:REF,ICF" "/DEBUG" "/LIBPATH:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps" "/LIBPATH:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\release\\deps" "/LIBPATH:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\build\\compiler_builtins-05da96f10ee08182\\out" "/LIBPATH:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-sysroot\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\libpanic_unwind-6ab08a78974024cf.rlib" "C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\libunwind-2869f0f2f2f58965.rlib" "C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\liblibc-328e7af81e8060e6.rlib" "C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\liballoc_system-b7d1cae0ef4f9328.rlib" "C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\liballoc-d98741ac693feb4c.rlib" "C:\\Users\\cyber\\AppData\\Local\\Temp\\rustc.CEA4nMZsBt0t\\libcore-c1274e4da0faaccf.rlib" "libcmt.lib" "/DLL" "/IMPLIB:C:\\Users\\cyber\\Rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-std\\x86_64-pc-windows-msvc\\release\\deps\\std-0dcc26fc3bac26c2.dll.lib"
  = note:    Creating library C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage0-std\x86_64-pc-windows-msvc\release\deps\std-0dcc26fc3bac26c2.dll.lib and object C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage0-std\x86_64-pc-windows-msvc\release\deps\std-0dcc26fc3bac26c2.dll.exp
          std-0dcc26fc3bac26c2.crate.allocator.rcgu.o : error LNK2019: unresolved external symbol __rdl_usable_size referenced in function __rust_usable_size
          std-0dcc26fc3bac26c2.crate.allocator.rcgu.o : error LNK2019: unresolved external symbol __rdl_alloc_excess referenced in function __rust_alloc_excess
          std-0dcc26fc3bac26c2.crate.allocator.rcgu.o : error LNK2019: unresolved external symbol __rdl_realloc_excess referenced in function __rust_realloc_excess
          std-0dcc26fc3bac26c2.crate.allocator.rcgu.o : error LNK2019: unresolved external symbol __rdl_grow_in_place referenced in function __rust_grow_in_place
          std-0dcc26fc3bac26c2.crate.allocator.rcgu.o : error LNK2019: unresolved external symbol __rdl_shrink_in_place referenced in function __rust_shrink_in_place
          libcore-c1274e4da0faaccf.rlib(core-c1274e4da0faaccf.core2-b236f49e4e6ab3c48c3cbb1a4de38b83.rs.rcgu.o) : error LNK2019: unresolved external symbol __udivti3 referenced in function _ZN4core3num14from_str_radix17h8fe975e8dc86c691E
          libcore-c1274e4da0faaccf.rlib(core-c1274e4da0faaccf.core0-b236f49e4e6ab3c48c3cbb1a4de38b83.rs.rcgu.o) : error LNK2001: unresolved external symbol __udivti3
          libcore-c1274e4da0faaccf.rlib(core-c1274e4da0faaccf.core2-b236f49e4e6ab3c48c3cbb1a4de38b83.rs.rcgu.o) : error LNK2019: unresolved external symbol __muloti4 referenced in function _ZN4core3num14from_str_radix17ha8e33a846b1be59fE
          C:\Users\cyber\Rust\rust\build\x86_64-pc-windows-msvc\stage0-std\x86_64-pc-windows-msvc\release\deps\std-0dcc26fc3bac26c2.dll : fatal error LNK1120: 7 unresolved externals


error: aborting due to previous error

error: Could not compile `std`.
