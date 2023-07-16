
[1700/1912] Linking CXX executable bin\llvm-cxxfilt.exe
FAILED: bin/llvm-cxxfilt.exe 
cmd.exe /C "cd . && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E vs_link_exe --intdir=tools\llvm-cxxfilt\CMakeFiles\llvm-cxxfilt.dir --manifests  -- C:\PROGRA~2\MI0E91~1.0\VC\bin\amd64\link.exe /nologo tools\llvm-cxxfilt\CMakeFiles\llvm-cxxfilt.dir\llvm-cxxfilt.cpp.obj tools\llvm-cxxfilt\CMakeFiles\llvm-cxxfilt.dir\__\__\resources\windows_version_resource.rc.res  /out:bin\llvm-cxxfilt.exe /implib:lib\llvm-cxxfilt.lib /pdb:bin\llvm-cxxfilt.pdb /version:0.0  /machine:x64 /STACK:10000000 /INCREMENTAL:NO /subsystem:console  lib\LLVM.lib kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib && cd ."
LINK : fatal error LNK1181: cannot open input file 'lib\LLVM.lib'
LINK failed. with 1181
