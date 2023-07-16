
2020-07-08T00:26:17.1279850Z stdout:
2020-07-08T00:26:17.1280176Z ------------------------------------------
2020-07-08T00:26:17.1280266Z 
2020-07-08T00:26:17.1280391Z ------------------------------------------
2020-07-08T00:26:17.1280520Z stderr:
2020-07-08T00:26:17.1280643Z ------------------------------------------
2020-07-08T00:26:17.1280782Z error: linking with `link.exe` failed: exit code: 1120
2020-07-08T00:26:17.1280910Z   |
2020-07-08T00:26:17.1281622Z   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.26.28801\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my-core.my_core.3a1fbbbh-cgu.0.rcgu.o" "/OUT:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my_core.dll" "/DEF:C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\rustcqBUT54\\lib.def" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my-core.41vnijj432q48xyz.rcgu.o" "/OPT:REF,ICF" "/DLL" "/IMPLIB:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary\\my_core.dll.lib" "/DEBUG" "/NATVIS:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\etc\\libstd.natvis" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\test\\rustdoc\\intra-link-prim-methods-external-core\\auxiliary" "/LIBPATH:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib"
2020-07-08T00:26:17.1282239Z   = note:    Creating library D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\rustdoc\intra-link-prim-methods-external-core\auxiliary\my_core.dll.lib and object D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\rustdoc\intra-link-prim-methods-external-core\auxiliary\my_core.dll.exp
2020-07-08T00:26:17.1282454Z           LINK : error LNK2001: unresolved external symbol _DllMainCRTStartup
2020-07-08T00:26:17.1282679Z           D:\a\rust\rust\build\x86_64-pc-windows-msvc\test\rustdoc\intra-link-prim-methods-external-core\auxiliary\my_core.dll : fatal error LNK1120: 1 unresolved externals
2020-07-08T00:26:17.1283011Z           
2020-07-08T00:26:17.1283071Z 
