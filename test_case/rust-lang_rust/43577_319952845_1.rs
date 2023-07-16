
[1698/1912] Linking CXX shared library bin\LTO.dll
FAILED: bin/LTO.dll lib/LTO.lib 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E vs_link_dll --intdir=tools\lto\CMakeFiles\LTO.dir --manifests  -- C:\PROGRA~2\MI0E91~1.0\VC\bin\amd64\link.exe /nologo tools\lto\CMakeFiles\LTO.dir\LTODisassembler.cpp.obj tools\lto\CMakeFiles\LTO.dir\lto.cpp.obj tools\lto\CMakeFiles\LTO.dir\__\__\resources\windows_version_resource.rc.res  /out:bin\LTO.dll /implib:lib\LTO.lib /pdb:bin\LTO.pdb /dll /version:0.0 /machine:x64 /INCREMENTAL:NO  /DEF:"C:/projects/rust/build/x86_64-pc-windows-msvc/llvm/build/tools/lto/LTO.def"  lib\LLVM.lib kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib  && cd ."
LINK : fatal error LNK1181: cannot open input file 'lib\LLVM.lib'
LINK failed. with 1181
