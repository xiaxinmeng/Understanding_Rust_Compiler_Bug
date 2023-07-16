
Microsoft (R) Windows Debugger Version 10.0.19528.1000 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: starship prompt
Starting directory: C:\Users\Chris

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff6`8c790000 00007ff6`8cc34000   starship-70f20c3ed1a6265e.exe
ModLoad: 00007ff9`406a0000 00007ff9`40890000   ntdll.dll
ModLoad: 00007ff9`3feb0000 00007ff9`3ff62000   C:\WINDOWS\System32\KERNEL32.DLL
ModLoad: 00007ff9`3d990000 00007ff9`3dc34000   C:\WINDOWS\System32\KERNELBASE.dll

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`faf00000 00007ff7`fafe0000   conhost.exe
ModLoad: 00007ff9`406a0000 00007ff9`40890000   ntdll.dll
ModLoad: 00007ff9`3feb0000 00007ff9`3ff62000   C:\WINDOWS\System32\KERNEL32.DLL
ModLoad: 00007ff9`3d990000 00007ff9`3dc34000   C:\WINDOWS\System32\KERNELBASE.dll
ModLoad: 00007ff9`3d8f0000 00007ff9`3d98e000   C:\WINDOWS\System32\msvcp_win.dll
ModLoad: 00007ff9`3d7f0000 00007ff9`3d8ea000   C:\WINDOWS\System32\ucrtbase.dll
ModLoad: 00007ff9`3e7f0000 00007ff9`3e899000   C:\WINDOWS\System32\shcore.dll
ModLoad: 00007ff9`3e8a0000 00007ff9`3e93e000   C:\WINDOWS\System32\msvcrt.dll
ModLoad: 00007ff9`3f4c0000 00007ff9`3f5e0000   C:\WINDOWS\System32\RPCRT4.dll
ModLoad: 00007ff9`3f7b0000 00007ff9`3fae5000   C:\WINDOWS\System32\combase.dll
ModLoad: 00007ff9`3d690000 00007ff9`3d710000   C:\WINDOWS\System32\bcryptPrimitives.dll
ModLoad: 00007ff9`3edb0000 00007ff9`3ee53000   C:\WINDOWS\System32\advapi32.dll
ModLoad: 00007ff9`3e940000 00007ff9`3e9d7000   C:\WINDOWS\System32\sechost.dll
(6304.61e4): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`4077119c cc              int     3
1:001> g
ModLoad: 00007ff9`3faf0000 00007ff9`3fc84000   C:\WINDOWS\System32\user32.dll
ModLoad: 00007ff9`3dcb0000 00007ff9`3dcd1000   C:\WINDOWS\System32\win32u.dll
ModLoad: 00007ff9`3e750000 00007ff9`3e776000   C:\WINDOWS\System32\GDI32.dll
ModLoad: 00007ff9`3de30000 00007ff9`3dfc5000   C:\WINDOWS\System32\gdi32full.dll
ModLoad: 00007ff9`3ef90000 00007ff9`3efbe000   C:\WINDOWS\System32\IMM32.DLL
ModLoad: 00007ff9`3ff70000 00007ff9`40656000   C:\WINDOWS\System32\shell32.dll
ModLoad: 00007ff9`3dc40000 00007ff9`3dc8a000   C:\WINDOWS\System32\cfgmgr32.dll
ModLoad: 00007ff9`3dfd0000 00007ff9`3e750000   C:\WINDOWS\System32\windows.storage.dll
ModLoad: 00007ff9`3d5c0000 00007ff9`3d5e3000   C:\WINDOWS\System32\profapi.dll
ModLoad: 00007ff9`3d570000 00007ff9`3d5ba000   C:\WINDOWS\System32\powrprof.dll
ModLoad: 00007ff9`3d560000 00007ff9`3d570000   C:\WINDOWS\System32\UMPDC.dll
ModLoad: 00007ff9`3ecc0000 00007ff9`3ed12000   C:\WINDOWS\System32\shlwapi.dll
ModLoad: 00007ff9`3d5f0000 00007ff9`3d601000   C:\WINDOWS\System32\kernel.appcore.dll
ModLoad: 00007ff9`3dc90000 00007ff9`3dca7000   C:\WINDOWS\System32\cryptsp.dll
ModLoad: 00007ff9`3b380000 00007ff9`3b419000   C:\WINDOWS\system32\uxtheme.dll
onecore\windows\core\console\open\src\interactivity\win32\systemconfigurationprovider.cpp(179)\conhost.exe!00007FF7FAF056B5: (caller: 00007FF7FAF060C3) LogHr(1) tid(62d8) 80004005 Unbekannter Fehler
onecore\windows\core\console\open\src\renderer\gdi\state.cpp(245)\conhost.exe!00007FF7FAF15477: (caller: 00007FF7FAF13FF7) LogHr(2) tid(52f8) 80004005 Unbekannter Fehler
ModLoad: 00007ff9`3fc90000 00007ff9`3fdc5000   C:\WINDOWS\System32\MSCTF.dll
ModLoad: 00007ff9`3ee60000 00007ff9`3ef25000   C:\WINDOWS\System32\OLEAUT32.dll
ModLoad: 00007ff9`28fb0000 00007ff9`29234000   C:\WINDOWS\WinSxS\amd64_microsoft.windows.common-controls_6595b64144ccf1df_6.0.18362.900_none_e6beb9d913147d17\comctl32.DLL
ModLoad: 00007ff9`3b8e0000 00007ff9`3b90d000   C:\WINDOWS\SYSTEM32\dwmapi.dll
ModLoad: 00007ff9`2a7e0000 00007ff9`2a87e000   C:\WINDOWS\System32\TextInputFramework.dll
ModLoad: 00007ff9`39d80000 00007ff9`39e54000   C:\WINDOWS\System32\CoreMessaging.dll
ModLoad: 00007ff9`37490000 00007ff9`377ba000   C:\WINDOWS\System32\CoreUIComponents.dll
ModLoad: 00007ff9`3c580000 00007ff9`3c5b1000   C:\WINDOWS\SYSTEM32\ntmarta.dll
ModLoad: 00007ff9`3a230000 00007ff9`3a383000   C:\WINDOWS\SYSTEM32\wintypes.dll
ModLoad: 000001f0`9bac0000 000001f0`9bc13000   C:\WINDOWS\SYSTEM32\wintypes.dll
ModLoad: 00007ff9`2db40000 00007ff9`2dde6000   C:\WINDOWS\System32\iertutil.dll
ModLoad: 00007ff9`3e9e0000 00007ff9`3ea82000   C:\WINDOWS\System32\clbcatq.dll
ModLoad: 00007ff9`3ea90000 00007ff9`3ebe7000   C:\WINDOWS\System32\ole32.dll
ModLoad: 00007ff9`3f7b0000 00007ff9`3fae5000   C:\WINDOWS\System32\combase.dll
ModLoad: 00007ff9`3d7f0000 00007ff9`3d8ea000   C:\WINDOWS\System32\ucrtbase.dll
ModLoad: 00007ff9`3f4c0000 00007ff9`3f5e0000   C:\WINDOWS\System32\RPCRT4.dll
ModLoad: 00007ff9`3d690000 00007ff9`3d710000   C:\WINDOWS\System32\bcryptPrimitives.dll
ModLoad: 00007ff9`3edb0000 00007ff9`3ee53000   C:\WINDOWS\System32\advapi32.dll
ModLoad: 00007ff9`3e8a0000 00007ff9`3e93e000   C:\WINDOWS\System32\msvcrt.dll
ModLoad: 00007ff9`3e940000 00007ff9`3e9d7000   C:\WINDOWS\System32\sechost.dll
ModLoad: 00007ff9`3e750000 00007ff9`3e776000   C:\WINDOWS\System32\GDI32.dll
ModLoad: 00007ff9`3dcb0000 00007ff9`3dcd1000   C:\WINDOWS\System32\win32u.dll
ModLoad: 00007ff9`3de30000 00007ff9`3dfc5000   C:\WINDOWS\System32\gdi32full.dll
ModLoad: 00007ff9`3d8f0000 00007ff9`3d98e000   C:\WINDOWS\System32\msvcp_win.dll
ModLoad: 00007ff9`3faf0000 00007ff9`3fc84000   C:\WINDOWS\System32\USER32.dll
ModLoad: 00007ff9`3dce0000 00007ff9`3de29000   C:\WINDOWS\System32\CRYPT32.dll
ModLoad: 00007ff9`3d610000 00007ff9`3d622000   C:\WINDOWS\System32\MSASN1.dll
ModLoad: 00007ff9`3ee60000 00007ff9`3ef25000   C:\WINDOWS\System32\OLEAUT32.dll
ModLoad: 00007ff9`3d570000 00007ff9`3d5ba000   C:\WINDOWS\System32\POWRPROF.dll
ModLoad: 00007ff9`29270000 00007ff9`29287000   C:\WINDOWS\SYSTEM32\NETAPI32.dll
ModLoad: 00007ff9`3caa0000 00007ff9`3cada000   C:\WINDOWS\SYSTEM32\IPHLPAPI.DLL
ModLoad: 00007ff9`3d560000 00007ff9`3d570000   C:\WINDOWS\System32\UMPDC.dll
ModLoad: 00007ff9`27e30000 00007ff9`27e78000   C:\WINDOWS\SYSTEM32\pdh.dll
ModLoad: 00007ff9`3eda0000 00007ff9`3eda8000   C:\WINDOWS\System32\PSAPI.DLL
ModLoad: 00007ff9`2c260000 00007ff9`2c26c000   C:\WINDOWS\SYSTEM32\Secur32.dll
ModLoad: 00007ff9`3f050000 00007ff9`3f4c0000   C:\WINDOWS\System32\SETUPAPI.dll
ModLoad: 00007ff9`3dc40000 00007ff9`3dc8a000   C:\WINDOWS\System32\cfgmgr32.dll
ModLoad: 00007ff9`3d710000 00007ff9`3d736000   C:\WINDOWS\System32\bcrypt.dll
ModLoad: 00007ff9`3ff70000 00007ff9`40656000   C:\WINDOWS\System32\SHELL32.dll
ModLoad: 00007ff9`3e7f0000 00007ff9`3e899000   C:\WINDOWS\System32\shcore.dll
ModLoad: 00007ff9`3dfd0000 00007ff9`3e750000   C:\WINDOWS\System32\windows.storage.dll
ModLoad: 00007ff9`3d5c0000 00007ff9`3d5e3000   C:\WINDOWS\System32\profapi.dll
ModLoad: 00007ff9`3ecc0000 00007ff9`3ed12000   C:\WINDOWS\System32\shlwapi.dll
ModLoad: 00007ff9`3d5f0000 00007ff9`3d601000   C:\WINDOWS\System32\kernel.appcore.dll
ModLoad: 00007ff9`3dc90000 00007ff9`3dca7000   C:\WINDOWS\System32\cryptsp.dll
ModLoad: 00007ff9`3ed30000 00007ff9`3ed9f000   C:\WINDOWS\System32\WS2_32.dll
ModLoad: 00007ff9`2c240000 00007ff9`2c256000   C:\WINDOWS\SYSTEM32\VCRUNTIME140.dll
ModLoad: 00007ff9`3cf00000 00007ff9`3cf0c000   C:\WINDOWS\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`3cbb0000 00007ff9`3cbbc000   C:\WINDOWS\SYSTEM32\NETUTILS.DLL
ModLoad: 00007ff9`362f0000 00007ff9`36308000   C:\WINDOWS\SYSTEM32\SAMCLI.DLL
ModLoad: 00007ff9`3d480000 00007ff9`3d4af000   C:\WINDOWS\SYSTEM32\SSPICLI.DLL
(5cc4.5ddc): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`4077119c cc              int     3
0:000> g
ModLoad: 00007ff9`3ef90000 00007ff9`3efbe000   C:\WINDOWS\System32\IMM32.DLL
(5cc4.5ddc): Access violation - code c0000005 (first chance)
First chance exceptions are reported before any exception handling.
This exception may be expected and handled.
*** WARNING: Unable to verify checksum for starship-70f20c3ed1a6265e.exe
starship_70f20c3ed1a6265e!Ordinal0+0x185915:
00007ff6`8c915915 c5fc2888c0000000 vmovaps ymm1,ymmword ptr [rax+0C0h] ds:00000178`afaa49b0=00
0:000> u @rip
starship_70f20c3ed1a6265e!Ordinal0+0x185915:
00007ff6`8c915915 c5fc2888c0000000 vmovaps ymm1,ymmword ptr [rax+0C0h]
00007ff6`8c91591d 488bb0c0000000  mov     rsi,qword ptr [rax+0C0h]
00007ff6`8c915924 c5fc2980c0000000 vmovaps ymmword ptr [rax+0C0h],ymm0
00007ff6`8c91592c c5fc294b20      vmovaps ymmword ptr [rbx+20h],ymm1
00007ff6`8c915931 4885f6          test    rsi,rsi
00007ff6`8c915934 0f8493000000    je      starship_70f20c3ed1a6265e!Ordinal0+0x1859cd (00007ff6`8c9159cd)
00007ff6`8c91593a 488b05876c2f00  mov     rax,qword ptr [starship_70f20c3ed1a6265e!git_libgit2_opts+0x141918 (00007ff6`8cc0c5c8)]
00007ff6`8c915941 4883f801        cmp     rax,1
0:000>
