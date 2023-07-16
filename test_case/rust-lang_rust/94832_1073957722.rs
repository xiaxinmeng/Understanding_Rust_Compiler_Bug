
Performing C++ SOURCE FILE Test CXX_SUPPORTS_CUSTOM_LINKER failed with the following output:
Change Dir: C:/msys64/home/we/rust/build/x86_64-pc-windows-gnu/llvm/build/CMakeFiles/CMakeTmp

Run Build Command(s):C:/ProgramData/chocolatey/bin/ninja.exe cmTC_60dd7 && [1/2] Building CXX object CMakeFiles/cmTC_60dd7.dir/src.cxx.obj

[2/2] Linking CXX executable cmTC_60dd7.exe

FAILED: cmTC_60dd7.exe 

cmd.exe /C "cd . && C:\msys64\mingw64\bin\g++.exe -ffunction-sections -fdata-sections -m64  -fuse-ld=lld -Wl,-Bsymbolic -static-libstdc++ CMakeFiles/cmTC_60dd7.dir/src.cxx.obj -o cmTC_60dd7.exe -Wl,--out-implib,libcmTC_60dd7.dll.a -Wl,--major-image-version,0,--minor-image-version,0  -lkernel32 -luser32 -lgdi32 -lwinspool -lshell32 -lole32 -loleaut32 -luuid -lcomdlg32 -ladvapi32 && cd ."

lld: error: unknown argument: -Bsymbolic

collect2.exe: error: ld returned 1 exit status
ninja: build stopped: subcommand failed.



Source file was:
int main() { return 0; }
