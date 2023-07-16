plain
[128/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Constants.cpp.obj
[129/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Core.cpp.obj
[130/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\IRPrintingPasses.cpp.obj
[131/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\GCStrategy.cpp.obj
[132/3009] Configuring NATIVE LLVM...
FAILED: NATIVE/CMakeCache.txt D:/a/rust/rust/build/aarch64-pc-windows-msvc/llvm/build/NATIVE/CMakeCache.txt 
cmd.exe /C "cd /D D:\a\rust\rust\build\aarch64-pc-windows-msvc\llvm\build\NATIVE && "C:\Program Files\CMake\bin\cmake.exe" -G Ninja -DCMAKE_MAKE_PROGRAM="D:/a/rust/rust/ninja/ninja.exe" -DCMAKE_C_COMPILER_LAUNCHER="" -DCMAKE_CXX_COMPILER_LAUNCHER="" -DCMAKE_C_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe -DCMAKE_CXX_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe D:/a/rust/rust/src/llvm-project/llvm -DLLVM_TARGET_IS_CROSSCOMPILE_HOST=TRUE -DLLVM_TARGETS_TO_BUILD="AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86;AVR;M68k" -DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD="AVR;M68k" -DLLVM_DEFAULT_TARGET_TRIPLE="aarch64-pc-windows-msvc" -DLLVM_TARGET_ARCH="aarch64" -DLLVM_ENABLE_PROJECTS="" -DLLVM_EXTERNAL_PROJECTS="" -DLLVM_ENABLE_RUNTIMES="" -DLLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN="OFF" -DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 14.0.1 with MSVC-like command-line
-- The ASM compiler identification is Clang with MSVC-like command-line
-- Found assembler: D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - failed
-- Check for working C compiler: D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe
-- Check for working C compiler: D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe - broken
CMake Error at C:/Program Files/CMake/share/cmake-3.23/Modules/CMakeTestCCompiler.cmake:69 (message):

    "D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe"

  is not able to compile a simple test program.
  is not able to compile a simple test program.

  It fails with the following output:

    Change Dir: D:/a/rust/rust/build/aarch64-pc-windows-msvc/llvm/build/NATIVE/CMakeFiles/CMakeTmp
    
    Run Build Command(s):D:/a/rust/rust/ninja/ninja.exe cmTC_51d30 && [1/2] Building C object CMakeFiles\cmTC_51d30.dir\testCCompiler.c.obj

    [2/2] Linking C executable cmTC_51d30.exe

    FAILED: cmTC_51d30.exe 

    cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E vs_link_exe --intdir=CMakeFiles\cmTC_51d30.dir --rc=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\rc.exe --mt=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\mt.exe --manifests  -- C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\MSVC\1429~1.301\bin\Hostx64\arm64\link.exe /nologo CMakeFiles\cmTC_51d30.dir\testCCompiler.c.obj  /out:cmTC_51d30.exe /implib:cmTC_51d30.lib /pdb:cmTC_51d30.pdb /version:0.0 /machine:x64  /debug /INCREMENTAL /subsystem:console  kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib && cd ."

    LINK Pass 1: command "C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\MSVC\1429~1.301\bin\Hostx64\arm64\link.exe /nologo CMakeFiles\cmTC_51d30.dir\testCCompiler.c.obj /out:cmTC_51d30.exe /implib:cmTC_51d30.lib /pdb:cmTC_51d30.pdb /version:0.0 /machine:x64 /debug /INCREMENTAL /subsystem:console kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib /MANIFEST /MANIFESTFILE:CMakeFiles\cmTC_51d30.dir/intermediate.manifest CMakeFiles\cmTC_51d30.dir/manifest.res" failed (exit code 1120) with the following output:

    LINK : error LNK2001: unresolved external symbol mainCRTStartup


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\kernel32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\user32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\gdi32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\winspool.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\shell32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\ole32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\oleaut32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\uuid.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\comdlg32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Windows Kits\10\lib\10.0.22000.0\um\arm64\advapi32.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\arm64\msvcrtd.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\lib\arm64\oldnames.lib : warning LNK4272: library machine type 'ARM64' conflicts with target machine type 'x64'


    cmTC_51d30.exe : fatal error LNK1120: 1 unresolved externals


    ninja: build stopped: subcommand failed.
    
    

  
  

  CMake will not be able to correctly generate this project.
Call Stack (most recent call first):
  CMakeLists.txt:49 (project)


-- Configuring incomplete, errors occurred!
See also "D:/a/rust/rust/build/aarch64-pc-windows-msvc/llvm/build/NATIVE/CMakeFiles/CMakeOutput.log".
See also "D:/a/rust/rust/build/aarch64-pc-windows-msvc/llvm/build/NATIVE/CMakeFiles/CMakeError.log".
[134/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\InlineAsm.cpp.obj
[135/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Globals.cpp.obj
[136/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Instruction.cpp.obj
[137/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Instructions.cpp.obj
[137/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Instructions.cpp.obj
[138/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Dominators.cpp.obj
[139/3009] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\IntrinsicInst.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
 finished in 53.971 seconds
Build completed unsuccessfully in 0:29:32
