
Microsoft (R) Windows Debugger Version 10.0.19528.1000 AMD64
Copyright (c) Microsoft Corporation. All rights reserved.

CommandLine: rustc  --crate-name build_script_build --edition=2018 C:\Users\Chris\.cargo\registry\src\github.com-1ecc6299db9ec823\sd-0.7.5\build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -Cembed-bitcode=no -C codegen-units=1 -C metadata=d011dbc6f5f1cce2 -C extra-filename=-d011dbc6f5f1cce2 --out-dir C:\Users\Chris\AppData\Local\Temp\cargo-installl0X1hR\release\build\sd-d011dbc6f5f1cce2 -L dependency=C:\Users\Chris\AppData\Local\Temp\cargo-installl0X1hR\release\deps --extern man=C:\Users\Chris\AppData\Local\Temp\cargo-installl0X1hR\release\deps\libman-b99848b80d918758.rlib --extern structopt=C:\Users\Chris\AppData\Local\Temp\cargo-installl0X1hR\release\deps\libstructopt-ee882b02652d4990.rlib --cap-lints allow -Clinker-flavor=lld-link -Ctarget-feature=+avx
Starting directory: C:\Users\Chris

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`02ce0000 00007ff7`03521000   rustup_init.exe
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
(36d0.2008): Break instruction exception - code 80000003 (first chance)
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
onecore\windows\core\console\open\src\interactivity\win32\systemconfigurationprovider.cpp(179)\conhost.exe!00007FF7FAF056B5: (caller: 00007FF7FAF060C3) LogHr(1) tid(2a94) 80004005 Unbekannter Fehler
onecore\windows\core\console\open\src\renderer\gdi\state.cpp(245)\conhost.exe!00007FF7FAF15477: (caller: 00007FF7FAF13FF7) LogHr(2) tid(5b4c) 80004005 Unbekannter Fehler
ModLoad: 00007ff9`3fc90000 00007ff9`3fdc5000   C:\WINDOWS\System32\MSCTF.dll
ModLoad: 00007ff9`3ee60000 00007ff9`3ef25000   C:\WINDOWS\System32\OLEAUT32.dll
ModLoad: 00007ff9`28fb0000 00007ff9`29234000   C:\WINDOWS\WinSxS\amd64_microsoft.windows.common-controls_6595b64144ccf1df_6.0.18362.900_none_e6beb9d913147d17\comctl32.DLL
ModLoad: 00007ff9`3b8e0000 00007ff9`3b90d000   C:\WINDOWS\SYSTEM32\dwmapi.dll
ModLoad: 00007ff9`2a7e0000 00007ff9`2a87e000   C:\WINDOWS\System32\TextInputFramework.dll
ModLoad: 00007ff9`37490000 00007ff9`377ba000   C:\WINDOWS\System32\CoreUIComponents.dll
ModLoad: 00007ff9`39d80000 00007ff9`39e54000   C:\WINDOWS\System32\CoreMessaging.dll
ModLoad: 00007ff9`3c580000 00007ff9`3c5b1000   C:\WINDOWS\SYSTEM32\ntmarta.dll
ModLoad: 00007ff9`3a230000 00007ff9`3a383000   C:\WINDOWS\SYSTEM32\wintypes.dll
ModLoad: 00000183`50180000 00000183`502d3000   C:\WINDOWS\SYSTEM32\wintypes.dll
ModLoad: 00000183`502e0000 00000183`50433000   C:\WINDOWS\SYSTEM32\wintypes.dll
ModLoad: 00007ff9`2db40000 00007ff9`2dde6000   C:\WINDOWS\System32\iertutil.dll
ModLoad: 00007ff9`3e9e0000 00007ff9`3ea82000   C:\WINDOWS\System32\clbcatq.dll
ModLoad: 00007ff9`3edb0000 00007ff9`3ee53000   C:\WINDOWS\System32\ADVAPI32.dll
ModLoad: 00007ff9`3e8a0000 00007ff9`3e93e000   C:\WINDOWS\System32\msvcrt.dll
ModLoad: 00007ff9`3e940000 00007ff9`3e9d7000   C:\WINDOWS\System32\sechost.dll
ModLoad: 00007ff9`3f4c0000 00007ff9`3f5e0000   C:\WINDOWS\System32\RPCRT4.dll
ModLoad: 00007ff9`3ea90000 00007ff9`3ebe7000   C:\WINDOWS\System32\ole32.dll
ModLoad: 00007ff9`3f7b0000 00007ff9`3fae5000   C:\WINDOWS\System32\combase.dll
ModLoad: 00007ff9`3d7f0000 00007ff9`3d8ea000   C:\WINDOWS\System32\ucrtbase.dll
ModLoad: 00007ff9`3d690000 00007ff9`3d710000   C:\WINDOWS\System32\bcryptPrimitives.dll
ModLoad: 00007ff9`3e750000 00007ff9`3e776000   C:\WINDOWS\System32\GDI32.dll
ModLoad: 00007ff9`3dcb0000 00007ff9`3dcd1000   C:\WINDOWS\System32\win32u.dll
ModLoad: 00007ff9`3de30000 00007ff9`3dfc5000   C:\WINDOWS\System32\gdi32full.dll
ModLoad: 00007ff9`3d8f0000 00007ff9`3d98e000   C:\WINDOWS\System32\msvcp_win.dll
ModLoad: 00007ff9`3faf0000 00007ff9`3fc84000   C:\WINDOWS\System32\USER32.dll
ModLoad: 00007ff9`3ee60000 00007ff9`3ef25000   C:\WINDOWS\System32\OLEAUT32.dll
ModLoad: 00007ff9`3ed30000 00007ff9`3ed9f000   C:\WINDOWS\System32\WS2_32.dll
ModLoad: 00007ff9`3dce0000 00007ff9`3de29000   C:\WINDOWS\System32\CRYPT32.dll
ModLoad: 00007ff9`3d610000 00007ff9`3d622000   C:\WINDOWS\System32\MSASN1.dll
ModLoad: 00007ff9`3ff70000 00007ff9`40656000   C:\WINDOWS\System32\SHELL32.dll
ModLoad: 00007ff9`3dc40000 00007ff9`3dc8a000   C:\WINDOWS\System32\cfgmgr32.dll
ModLoad: 00007ff9`2c260000 00007ff9`2c26c000   C:\WINDOWS\SYSTEM32\Secur32.dll
ModLoad: 00007ff9`3e7f0000 00007ff9`3e899000   C:\WINDOWS\System32\shcore.dll
ModLoad: 00007ff9`3dfd0000 00007ff9`3e750000   C:\WINDOWS\System32\windows.storage.dll
ModLoad: 00007ff9`3d5c0000 00007ff9`3d5e3000   C:\WINDOWS\System32\profapi.dll
ModLoad: 00007ff9`3d570000 00007ff9`3d5ba000   C:\WINDOWS\System32\powrprof.dll
ModLoad: 00007ff9`3d560000 00007ff9`3d570000   C:\WINDOWS\System32\UMPDC.dll
ModLoad: 00007ff9`3ecc0000 00007ff9`3ed12000   C:\WINDOWS\System32\shlwapi.dll
ModLoad: 00007ff9`3d5f0000 00007ff9`3d601000   C:\WINDOWS\System32\kernel.appcore.dll
ModLoad: 00007ff9`3dc90000 00007ff9`3dca7000   C:\WINDOWS\System32\cryptsp.dll
ModLoad: 00007ff9`3d450000 00007ff9`3d475000   C:\WINDOWS\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`3cf00000 00007ff9`3cf0c000   C:\WINDOWS\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`3d480000 00007ff9`3d4af000   C:\WINDOWS\SYSTEM32\SSPICLI.DLL
(66c.5e8c): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`4077119c cc              int     3
0:000> g
ModLoad: 00007ff9`3ef90000 00007ff9`3efbe000   C:\WINDOWS\System32\IMM32.DLL

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff6`75120000 00007ff6`7513b000   rustc_binary-cf2345702275a4fd.exe
ModLoad: 00007ff9`406a0000 00007ff9`40890000   ntdll.dll
ModLoad: 00007ff9`3feb0000 00007ff9`3ff62000   C:\WINDOWS\System32\KERNEL32.DLL
ModLoad: 00007ff9`3d990000 00007ff9`3dc34000   C:\WINDOWS\System32\KERNELBASE.dll
ModLoad: 00007ff8`ed8e0000 00007ff8`edbda000   C:\Users\Chris\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\std-36286308a8d35fa4.dll
ModLoad: 00007ff9`3edb0000 00007ff9`3ee53000   C:\WINDOWS\System32\ADVAPI32.dll
ModLoad: 00007ff8`a4b60000 00007ff8`aae2c000   C:\Users\Chris\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\bin\rustc_driver-9cb24e1c020a8fb6.dll
ModLoad: 00007ff9`3eda0000 00007ff9`3eda8000   C:\WINDOWS\System32\PSAPI.DLL
ModLoad: 00007ff9`3e8a0000 00007ff9`3e93e000   C:\WINDOWS\System32\msvcrt.dll
ModLoad: 00007ff9`3ff70000 00007ff9`40656000   C:\WINDOWS\System32\SHELL32.dll
ModLoad: 00007ff9`3e940000 00007ff9`3e9d7000   C:\WINDOWS\System32\sechost.dll
ModLoad: 00007ff9`3d7f0000 00007ff9`3d8ea000   C:\WINDOWS\System32\ucrtbase.dll
ModLoad: 00007ff9`3f4c0000 00007ff9`3f5e0000   C:\WINDOWS\System32\RPCRT4.dll
ModLoad: 00007ff9`3dc40000 00007ff9`3dc8a000   C:\WINDOWS\System32\cfgmgr32.dll
ModLoad: 00007ff9`3ed30000 00007ff9`3ed9f000   C:\WINDOWS\System32\WS2_32.dll
ModLoad: 00007ff9`3e7f0000 00007ff9`3e899000   C:\WINDOWS\System32\shcore.dll
ModLoad: 00007ff9`3f7b0000 00007ff9`3fae5000   C:\WINDOWS\System32\combase.dll
ModLoad: 00007ff9`3d450000 00007ff9`3d475000   C:\WINDOWS\SYSTEM32\USERENV.dll
ModLoad: 00007ff9`3d690000 00007ff9`3d710000   C:\WINDOWS\System32\bcryptPrimitives.dll
ModLoad: 00007ff9`3d5c0000 00007ff9`3d5e3000   C:\WINDOWS\System32\profapi.dll
ModLoad: 00007ff9`3dfd0000 00007ff9`3e750000   C:\WINDOWS\System32\windows.storage.dll
ModLoad: 00007ff9`3d8f0000 00007ff9`3d98e000   C:\WINDOWS\System32\msvcp_win.dll
ModLoad: 00007ff9`3cf00000 00007ff9`3cf0c000   C:\WINDOWS\SYSTEM32\CRYPTBASE.DLL
ModLoad: 00007ff9`3d570000 00007ff9`3d5ba000   C:\WINDOWS\System32\powrprof.dll
ModLoad: 00007ff9`3d560000 00007ff9`3d570000   C:\WINDOWS\System32\UMPDC.dll
ModLoad: 00007ff9`3ecc0000 00007ff9`3ed12000   C:\WINDOWS\System32\shlwapi.dll
ModLoad: 00007ff9`3e750000 00007ff9`3e776000   C:\WINDOWS\System32\GDI32.dll
ModLoad: 00007ff9`3dcb0000 00007ff9`3dcd1000   C:\WINDOWS\System32\win32u.dll
ModLoad: 00007ff9`3de30000 00007ff9`3dfc5000   C:\WINDOWS\System32\gdi32full.dll
ModLoad: 00007ff9`3faf0000 00007ff9`3fc84000   C:\WINDOWS\System32\USER32.dll
ModLoad: 00007ff9`3d5f0000 00007ff9`3d601000   C:\WINDOWS\System32\kernel.appcore.dll
ModLoad: 00007ff9`3dc90000 00007ff9`3dca7000   C:\WINDOWS\System32\cryptsp.dll
ModLoad: 00007ff9`3ea90000 00007ff9`3ebe7000   C:\WINDOWS\System32\ole32.dll
ModLoad: 00007ff9`3ee60000 00007ff9`3ef25000   C:\WINDOWS\System32\OLEAUT32.dll
(2430.6038): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x30:
00007ff9`4077119c cc              int     3
2:011> g
ModLoad: 00007ff9`3ef90000 00007ff9`3efbe000   C:\WINDOWS\System32\IMM32.DLL
ModLoad: 00007ff8`eba80000 00007ff8`ebcae000   \\?\C:\Users\Chris\AppData\Local\Temp\cargo-installl0X1hR\release\deps\structopt_derive-9f1d169c48bf0bb3.dll
ModLoad: 00007ff9`2c240000 00007ff9`2c256000   C:\WINDOWS\SYSTEM32\VCRUNTIME140.dll
(2430.5afc): Access violation - code c0000005 (first chance)
First chance exceptions are reported before any exception handling.
This exception may be expected and handled.
*** WARNING: Unable to verify checksum for \\?\C:\Users\Chris\AppData\Local\Temp\cargo-installl0X1hR\release\deps\structopt_derive-9f1d169c48bf0bb3.dll
structopt_derive_9f1d169c48bf0bb3!_rust_realloc+0x3fe:
00007ff8`ebae654e c5fc288120000000 vmovaps ymm0,ymmword ptr [rcx+20h] ds:000002c8`178b5db0=00
2:015> u @rip
structopt_derive_9f1d169c48bf0bb3!_rust_realloc+0x3fe:
00007ff8`ebae654e c5fc288120000000 vmovaps ymm0,ymmword ptr [rcx+20h]
00007ff8`ebae6556 4883b92000000000 cmp     qword ptr [rcx+20h],0
00007ff8`ebae655e c5fc280d3af21100 vmovaps ymm1,ymmword ptr [structopt_derive_9f1d169c48bf0bb3!_ymm (00007ff8`ebc057a0)]
00007ff8`ebae6566 488b8130000000  mov     rax,qword ptr [rcx+30h]
00007ff8`ebae656d c5fc298920000000 vmovaps ymmword ptr [rcx+20h],ymm1
00007ff8`ebae6575 c5fc11442428    vmovups ymmword ptr [rsp+28h],ymm0
00007ff8`ebae657b c5f8288140000000 vmovaps xmm0,xmmword ptr [rcx+40h]
00007ff8`ebae6583 c5f811442448    vmovups xmmword ptr [rsp+48h],xmm0
2:015>
