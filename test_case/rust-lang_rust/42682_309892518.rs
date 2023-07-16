
[01:33:02] error: linking with `link.exe` failed: exit code: 1120
[01:33:02]   |
[01:33:02]   = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "
C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-pass\\smallest-hello-world.0.o" "/OUT:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-pass\\smallest-hello-world.stage2-x86_64-pc-windows-msvc.exe" "/OPT:REF,ICF" "
/DEBUG" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-pass" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\test\\run-pass\\smallest-hello-world.stage2-x86_64-pc-windows-msvc.run-pass.libaux" "/LIBPAT
H:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "/LIBPATH:C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\projects\\rust\\build\\x86_64-pc-windows-
msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc_system-ace597b8fd7407ce.rlib" "C:\\projects\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-216e0f775a886879.rlib"
[01:33:02]   = note: smallest-hello-world.0.o : error LNK2019: unresolved external symbol puts referenced in function main
[01:33:02]           LINK : error LNK2001: unresolved external symbol mainCRTStartup
[01:33:02]           C:\projects\rust\build\x86_64-pc-windows-msvc\test\run-pass\smallest-hello-world.stage2-x86_64-pc-windows-msvc.exe : fatal error LNK1120: 2 unresolved externals
[01:33:02]
[01:33:02]
[01:33:02] error: aborting due to previous error(s)
[01:33:02]
