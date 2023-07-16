
2020-07-08T16:07:42.9417000Z ------------------------------------------
2020-07-08T16:07:42.9417333Z error: linking with `link.exe` failed: exit code: 1120
2020-07-08T16:07:42.9417576Z   |
2020-07-08T16:07:42.9417780Z thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:344:22
2020-07-08T16:07:42.9417955Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-08T16:07:42.9418656Z   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.26.28801\\bin\\HostX64\\x86\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LARGEADDRESSAWARE" "/SAFESEH" "/LIBPATH:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my-core.my_core.3a1fbbbh-cgu.0.rcgu.o" "/OUT:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my_core.dll" "/DEF:C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcBZEzZW\\lib.def" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my-core.41vnijj432q48xyz.rcgu.o" "/OPT:REF,ICF" "/DLL" "/IMPLIB:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my_core.dll.lib" "/DEBUG" "/NATVIS:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\libstd.natvis" "/LIBPATH:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "/LIBPATH:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary" "/LIBPATH:D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib"
2020-07-08T16:07:42.9419409Z   = note:    Creating library D:\a\rust\rust\build\i686-pc-windows-msvc\test\rustdoc\intra-link-prim-methods-external-core\auxiliary\my_core.dll.lib and object D:\a\rust\rust\build\i686-pc-windows-msvc\test\rustdoc\intra-link-prim-methods-external-core\auxiliary\my_core.dll.exp
2020-07-08T16:07:42.9419810Z           LINK : error LNK2001: unresolved external symbol __DllMainCRTStartup@12
2020-07-08T16:07:42.9420032Z           D:\a\rust\rust\build\i686-pc-windows-msvc\test\rustdoc\intra-link-prim-methods-external-core\auxiliary\my_core.dll : fatal error LNK1120: 1 unresolved externals
2020-07-08T16:07:42.9420212Z           
2020-07-08T16:07:42.9420299Z 
2020-07-08T16:07:42.9420591Z error: aborting due to previous error
2020-07-08T16:07:42.9420658Z 
2020-07-08T16:07:42.9420916Z 
2020-07-08T16:07:42.9421219Z ------------------------------------------
2020-07-08T16:07:42.9421309Z 
2020-07-08T16:07:42.9421601Z 
2020-07-08T16:07:42.9421681Z 
2020-07-08T16:07:42.9421964Z failures:
2020-07-08T16:07:42.9422103Z     [rustdoc] rustdoc\intra-link-prim-methods-external-core.rs
2020-07-08T16:07:42.9422179Z 
2020-07-08T16:07:42.9422325Z test result: FAILED. 344 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
