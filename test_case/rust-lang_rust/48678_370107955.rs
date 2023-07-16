
[01:36:47] 
[01:36:47] ---- cargo_compile_duplicate_build_targets stdout ----
[01:36:47] 	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
[01:36:47] thread 'cargo_compile_duplicate_build_targets' panicked at '
[01:36:47] Expected: execs
[01:36:47]     but: exited with exit code: 101
[01:36:47] --- stdout
[01:36:47] 
[01:36:47] --- stderr
[01:36:47] warning: file found to be present in multiple build targets: C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t17\foo\src\main.rs
[01:36:47]    Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t17/foo)
[01:36:47] error: linking with `link.exe` failed: exit code: 1120
[01:36:47]   |
[01:36:47]   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t17\\foo\\target\\debug\\deps\\main.main0.rcgu.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t17\\foo\\target\\debug\\deps\\main.dll" "/DEF:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\lib.def" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t17\\foo\\target\\debug\\deps\\main.crate.metadata.rcgu.o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t17\\foo\\target\\debug\\deps\\main.crate.allocator.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t17\\foo\\target\\debug\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\libstd-eb14c866c7d8079e.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\libpanic_unwind-d9e3c31294ee5a53.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\libunwind-2e66ebee4d02b6cc.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\liblibc-8cbf826e20532578.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\liballoc_system-1049e480f8c93d4c.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\liballoc-57066655dccc594b.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\libstd_unicode-5e9160d868db3b01.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\libcore-4426d4163f2204a4.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.yQF1cJq2iIUu\\libcompiler_builtins-57771d1755e2c2fb.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib" "/DLL" "/IMPLIB:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t17\\foo\\target\\debug\\deps\\main.dll.lib"
[01:36:47]   = note: lib.def : error LNK2001: unresolved external symbol rust_metadata_std_30391d5a7f5739392c5f92402ed165a3
[01:36:47]           C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t17\foo\target\debug\deps\main.dll.lib : fatal error LNK1120: 1 unresolved externals
[01:36:47]           
[01:36:47] 
[01:36:47] error: aborting due to previous error
[01:36:47] 
[01:36:47] error: Could not compile `foo`.
[01:36:47] 
[01:36:47] To learn more, run the command again with --verbose.
[01:36:47] ', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:13
[01:36:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:36:47] 
[01:36:47] ---- example_as_dylib stdout ----
[01:36:47] 	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build --example=ex`
[01:36:47] thread 'example_as_dylib' panicked at '
[01:36:47] Expected: execs
[01:36:47]     but: exited with exit code: 101
[01:36:47] --- stdout
[01:36:47] 
[01:36:47] --- stderr
[01:36:47]    Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t65/foo)
[01:36:47] error: linking with `link.exe` failed: exit code: 1120
[01:36:47]   |
[01:36:47]   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t65\\foo\\target\\debug\\examples\\ex-c5c731ad99668b59.ex0.rcgu.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t65\\foo\\target\\debug\\examples\\ex-c5c731ad99668b59.dll" "/DEF:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\lib.def" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t65\\foo\\target\\debug\\examples\\ex-c5c731ad99668b59.crate.metadata.rcgu.o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t65\\foo\\target\\debug\\examples\\ex-c5c731ad99668b59.crate.allocator.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t65\\foo\\target\\debug\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\libstd-eb14c866c7d8079e.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\libpanic_unwind-d9e3c31294ee5a53.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\libunwind-2e66ebee4d02b6cc.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\liblibc-8cbf826e20532578.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\liballoc_system-1049e480f8c93d4c.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\liballoc-57066655dccc594b.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\libstd_unicode-5e9160d868db3b01.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\libcore-4426d4163f2204a4.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.SnODzd3UO6lX\\libcompiler_builtins-57771d1755e2c2fb.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib" "/DLL" "/IMPLIB:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t65\\foo\\target\\debug\\examples\\ex-c5c731ad99668b59.dll.lib"
[01:36:47]   = note: lib.def : error LNK2001: unresolved external symbol rust_metadata_std_30391d5a7f5739392c5f92402ed165a3
[01:36:47]           C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t65\foo\target\debug\examples\ex-c5c731ad99668b59.dll.lib : fatal error LNK1120: 1 unresolved externals
[01:36:47]           
[01:36:47] 
[01:36:47] error: aborting due to previous error
[01:36:47] 
[01:36:47] error: Could not compile `foo`.
[01:36:47] 
[01:36:47] To learn more, run the command again with --verbose.
[01:36:47] ', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:13
[01:36:47] 
[01:36:47] ---- many_crate_types_correct stdout ----
[01:36:47] 	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
[01:36:47] thread 'many_crate_types_correct' panicked at '
[01:36:47] Expected: execs
[01:36:47]     but: exited with exit code: 101
[01:36:47] --- stdout
[01:36:47] 
[01:36:47] --- stderr
[01:36:47]    Compiling foo v0.5.0 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t95/foo)
[01:36:47] error: linking with `link.exe` failed: exit code: 1120
[01:36:47]   |
[01:36:47]   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t95\\foo\\target\\debug\\deps\\foo.foo0.rcgu.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t95\\foo\\target\\debug\\deps\\foo.dll" "/DEF:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\lib.def" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t95\\foo\\target\\debug\\deps\\foo.crate.metadata.rcgu.o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t95\\foo\\target\\debug\\deps\\foo.crate.allocator.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t95\\foo\\target\\debug\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\libstd-eb14c866c7d8079e.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\libpanic_unwind-d9e3c31294ee5a53.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\libunwind-2e66ebee4d02b6cc.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\liblibc-8cbf826e20532578.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\liballoc_system-1049e480f8c93d4c.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\liballoc-57066655dccc594b.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\libstd_unicode-5e9160d868db3b01.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\libcore-4426d4163f2204a4.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.DFxtdfeQ9lBG\\libcompiler_builtins-57771d1755e2c2fb.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib" "/DLL" "/IMPLIB:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t95\\foo\\target\\debug\\deps\\foo.dll.lib"
[01:36:47]   = note: lib.def : error LNK2001: unresolved external symbol rust_metadata_std_30391d5a7f5739392c5f92402ed165a3
[01:36:47]           C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t95\foo\target\debug\deps\foo.dll.lib : fatal error LNK1120: 1 unresolved externals
[01:36:47]           
[01:36:47] 
[01:36:47] error: aborting due to previous error
[01:36:47] 
[01:36:47] error: Could not compile `foo`.
[01:36:47] 
[01:36:47] To learn more, run the command again with --verbose.
[01:36:47] ', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:13
[01:36:47] 
[01:36:47] ---- many_crate_types_old_style_lib_location stdout ----
[01:36:47] 	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build`
[01:36:47] thread 'many_crate_types_old_style_lib_location' panicked at '
[01:36:47] Expected: execs
[01:36:47]     but: exited with exit code: 101
[01:36:47] --- stdout
[01:36:47] 
[01:36:47] --- stderr
[01:36:47] warning: path `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t96\foo\src\foo.rs` was erroneously implicitly accepted for library `foo`,
[01:36:47] please rename the file to `src/lib.rs` or set lib.path in Cargo.toml
[01:36:47]    Compiling foo v0.5.0 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t96/foo)
[01:36:47] error: linking with `link.exe` failed: exit code: 1120
[01:36:47]   |
[01:36:47]   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t96\\foo\\target\\debug\\deps\\foo.foo0.rcgu.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t96\\foo\\target\\debug\\deps\\foo.dll" "/DEF:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\lib.def" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t96\\foo\\target\\debug\\deps\\foo.crate.metadata.rcgu.o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t96\\foo\\target\\debug\\deps\\foo.crate.allocator.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t96\\foo\\target\\debug\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\libstd-eb14c866c7d8079e.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\libpanic_unwind-d9e3c31294ee5a53.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\libunwind-2e66ebee4d02b6cc.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\liblibc-8cbf826e20532578.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\liballoc_system-1049e480f8c93d4c.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\liballoc-57066655dccc594b.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\libstd_unicode-5e9160d868db3b01.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\libcore-4426d4163f2204a4.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.Kdp3SSlUEtqL\\libcompiler_builtins-57771d1755e2c2fb.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib" "/DLL" "/IMPLIB:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t96\\foo\\target\\debug\\deps\\foo.dll.lib"
[01:36:47]   = note: lib.def : error LNK2001: unresolved external symbol rust_metadata_std_30391d5a7f5739392c5f92402ed165a3
[01:36:47]           C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t96\foo\target\debug\deps\foo.dll.lib : fatal error LNK1120: 1 unresolved externals
[01:36:47]           
[01:36:47] 
[01:36:47] error: aborting due to previous error
[01:36:47] 
[01:36:47] error: Could not compile `foo`.
[01:36:47] 
[01:36:47] To learn more, run the command again with --verbose.
[01:36:47] ', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:13
[01:36:47] 
[01:36:47] ---- predictable_filenames stdout ----
[01:36:47] 	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v`
[01:36:47] thread 'predictable_filenames' panicked at '
[01:36:47] Expected: execs
[01:36:47]     but: exited with exit code: 101
[01:36:47] --- stdout
[01:36:47] 
[01:36:47] --- stderr
[01:36:47]    Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t106/foo)
[01:36:47]      Running `rustc --crate-name foo src\lib.rs --crate-type dylib --crate-type rlib --emit=dep-info,link -C debuginfo=2 -C metadata=4ee8a8298a9c578d --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t106\foo\target\debug\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t106\foo\target\debug\deps`
[01:36:47] error: linking with `link.exe` failed: exit code: 1120
[01:36:47]   |
[01:36:47]   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t106\\foo\\target\\debug\\deps\\foo.foo0.rcgu.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t106\\foo\\target\\debug\\deps\\foo.dll" "/DEF:C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\lib.def" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t106\\foo\\target\\debug\\deps\\foo.crate.metadata.rcgu.o" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t106\\foo\\target\\debug\\deps\\foo.crate.allocator.rcgu.o" "/OPT:REF,NOICF" "/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t106\\foo\\target\\debug\\deps" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\libstd-eb14c866c7d8079e.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\libpanic_unwind-d9e3c31294ee5a53.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\libunwind-2e66ebee4d02b6cc.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\liblibc-8cbf826e20532578.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\liballoc_system-1049e480f8c93d4c.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\liballoc-57066655dccc594b.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\libstd_unicode-5e9160d868db3b01.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\libcore-4426d4163f2204a4.rlib" "C:\\Users\\appveyor\\AppData\\Local\\Temp\\1\\rustc.ajFLgoONWQCx\\libcompiler_builtins-57771d1755e2c2fb.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib" "/DLL" "/IMPLIB:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\cit\\t106\\foo\\target\\debug\\deps\\foo.dll.lib"
[01:36:47]   = note: lib.def : error LNK2001: unresolved external symbol rust_metadata_std_30391d5a7f5739392c5f92402ed165a3
[01:36:47]           C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t106\foo\target\debug\deps\foo.dll.lib : fatal error LNK1120: 1 unresolved externals
[01:36:47]           
[01:36:47] 
[01:36:47] error: aborting due to previous error
[01:36:47] 
[01:36:47] error: Could not compile `foo`.
[01:36:47] 
[01:36:47] Caused by:
[01:36:47]   process didn't exit successfully: `rustc --crate-name foo src\lib.rs --crate-type dylib --crate-type rlib --emit=dep-info,link -C debuginfo=2 -C metadata=4ee8a8298a9c578d --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t106\foo\target\debug\deps -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t106\foo\target\debug\deps` (exit code: 101)
[01:36:47] ', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:13
[01:36:47] 
[01:36:47] 
[01:36:47] failures:
[01:36:47]     cargo_compile_duplicate_build_targets
[01:36:47]     example_as_dylib
[01:36:47]     many_crate_types_correct
[01:36:47]     many_crate_types_old_style_lib_location
[01:36:47]     predictable_filenames
[01:36:47] 
[01:36:47] test result: FAILED. 126 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
[01:36:47] 
[01:36:47] error: test failed, to rerun pass '--test build'
[01:36:47] 
[01:36:47] 
[01:36:47] command did not execute successfully: "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "--release" "--locked" "--color" "always" "--manifest-path" "C:\\projects\\rust\\src/tools/cargo/Cargo.toml"
[01:36:47] expected success, got: exit code: 101
[01:36:47] 
[01:36:47] 
[01:36:47] failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap test src/tools/cargo src/tools/cargotest src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty
[01:36:47] Build completed unsuccessfully in 0:16:29
