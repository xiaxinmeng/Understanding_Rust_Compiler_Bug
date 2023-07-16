
[00:13:08]   The C compiler "C:/projects/rust/build/bootstrap/debug/sccache-plus-cl.exe"
[00:13:08]   is not able to compile a simple test program.
[00:13:08] 
[00:13:08]   It fails with the following output:
[00:13:08] 
[00:13:08]    Change Dir: C:/projects/rust/build/i686-pc-windows-msvc/llvm/build/CMakeFiles/CMakeTmp
[00:13:08] 
[00:13:08]   
[00:13:08] 
[00:13:08]   Run Build Command:"C:/projects/rust/ninja.exe" "cmTC_1269f"
[00:13:08] 
[00:13:08]   [1/2] Building C object CMakeFiles\cmTC_1269f.dir\testCCompiler.c.obj
[00:13:08] 
[00:13:08]   cl : Command line warning D9025 : overriding '/MD' with '/MDd'
[00:13:08] 
[00:13:08]   cl : Command line warning D9025 : overriding '/MD' with '/MDd'
[00:13:08] 
[00:13:08]   [2/2] Linking C executable cmTC_1269f.exe
[00:13:08] 
[00:13:08]   FAILED: cmTC_1269f.exe 
[00:13:08] 
[00:13:08]   cmd.exe /C "cd .  && "C:\Program Files (x86)\CMake\bin\cmake.exe" -E
[00:13:08]   vs_link_exe --intdir=CMakeFiles\cmTC_1269f.dir --manifests --
[00:13:08]   C:\PROGRA~2\MI0E91~1.0\VC\bin\amd64\link.exe /nologo
[00:13:08]   CMakeFiles\cmTC_1269f.dir\testCCompiler.c.obj /out:cmTC_1269f.exe
[00:13:08]   /implib:cmTC_1269f.lib /pdb:cmTC_1269f.pdb /version:0.0 /machine:x64 /debug
[00:13:08]   /INCREMENTAL /subsystem:console kernel32.lib user32.lib gdi32.lib
[00:13:08]   winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib
[00:13:08]   advapi32.lib && cd ."
[00:13:08] 
[00:13:08]   testCCompiler.c.obj : error LNK2001: unresolved external symbol
[00:13:08]   _RTC_InitBase

[00:13:08] 
[00:13:08]   testCCompiler.c.obj : error LNK2001: unresolved external symbol
[00:13:08]   _RTC_Shutdown

[00:13:08] 
[00:13:08]   LINK : error LNK2001: unresolved external symbol mainCRTStartup

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\kernel32.lib
[00:13:08]   : warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\user32.lib :
[00:13:08]   warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\gdi32.lib :
[00:13:08]   warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\winspool.lib
[00:13:08]   : warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\shell32.lib
[00:13:08]   : warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\ole32.lib :
[00:13:08]   warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\oleaut32.lib
[00:13:08]   : warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\comdlg32.lib
[00:13:08]   : warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Windows Kits\10\lib\10.0.14393.0\um\x86\advapi32.lib
[00:13:08]   : warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\lib\MSVCRTD.lib :
[00:13:08]   warning LNK4272: library machine type 'X86' conflicts with target machine
[00:13:08]   type 'x64'

[00:13:08] 
[00:13:08]   cmTC_1269f.exe : fatal error LNK1120: 3 unresolved externals

[00:13:08] 
[00:13:08]   LINK Pass 1 failed.  with 1120
[00:13:08] 
[00:13:08]   ninja: build stopped: subcommand failed.
