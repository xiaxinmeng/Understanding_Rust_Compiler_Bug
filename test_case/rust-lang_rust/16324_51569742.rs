
D:\scratchpad>gdb hello.exe
GNU gdb (GDB) 7.6
Copyright (C) 2013 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "i686-w64-mingw32".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from D:\scratchpad\hello.exe...done.
(gdb) run
Starting program: D:\scratchpad\hello.exe
[New Thread 9992.0x245c]
warning: [9308] [SetupHookConfiguration] Performing Hook Feature Checks.

warning: [9308] [CDetourHelpers::CachedGetCurrentAppName] ProcessName[\Device\Ha
rddiskVolume2\scratchpad\hello.exe]

warning: [9308] [PipeComms::Disconnect] [9308]PipeComms: [] Disconnecting [FFFFF
FFF].

warning: [9308] [PipeComms::Do_Connection] [9308]PipeComms: Access - [c0000000]
[\\.\pipe\appsense_ConfigPipe_D3EA4760-A0CC-4d00-AA80-B0D747311668].

warning: [9308] [PipeComms::Do_SendMessage] [9308]PipeComms: [\\.\pipe\appsense_
ConfigPipe_D3EA4760-A0CC-4d00-AA80-B0D747311668] SendMessage result = [1] [0].

warning: [9308] [PipeComms::Do_ReceiveMessage] [9308]PipeComms: [\\.\pipe\appsen
se_ConfigPipe_D3EA4760-A0CC-4d00-AA80-B0D747311668] ReceiveMessage result = [1]
[0].

warning: [9308] [SetupHookConfiguration] Features for [hello.exe], NET = [0], UR
M = [1]: PDM = [0].

warning: [9308] [PipeComms::Disconnect] [9308]PipeComms: [\\.\pipe\appsense_Conf
igPipe_D3EA4760-A0CC-4d00-AA80-B0D747311668] Disconnecting [00000078].

warning: [9308] [PipeComms::Disconnect] [9308]PipeComms: [\\.\pipe\appsense_Conf
igPipe_D3EA4760-A0CC-4d00-AA80-B0D747311668] Disconnecting [FFFFFFFF].

warning: [9308] [CDetourCreateProcess::Load] CoGetObject - attempt to hook on Vi
sta and above OS

warning: [9308] [CDetourCreateProcess::Load] Hooked detour CoGetObject

warning: [9308] [`anonymous-namespace'::IsCurrentAppOutlook] match[0]

warning: [9308] [CDetourCreateProcess::Load] Hooked detour ShellExecuteExW

warning: [9308] [CDetourOpenSave::Load] CoCreateInstance & CoGetClassObject - at
tempt to hook on Vista and above OS

warning: [9308] [CDetourOpenSave::Load] Hooked detour CoCreateInstance

warning: [9308] [CDetourOpenSave::Load] Hooked detour CoGetClassObject

warning: [9308] [CDetourOpenSave::Load] Hooked detour GetOpenFileNameA

warning: [9308] [CDetourOpenSave::Load] Hooked detour GetOpenFileNameW

warning: [9308] [CDetourOpenSave::Load] Hooked detour GetSaveFileNameA

warning: [9308] [CDetourOpenSave::Load] Hooked detour GetSaveFileNameW

warning: [9308] [`anonymous-namespace'::IsCurrentAppIExplore] Current Process Na
me is \device\harddiskvolume2\scratchpad\hello.exe

warning: [9308] [`anonymous-namespace'::IsCurrentAppIExplore] Mached I.E name:0

warning: [9308] [IsImageExplorer] ProcessName[\Device\HarddiskVolume2\scratchpad
\hello.exe]

warning: [9308] [IsImageExplorer] match[0]

warning: [9308] [CCreateProcessCtrl::Start] CreateProcessCtrl::Start()

warning: [9308] [CCreateProcessCtrl::AttachFunctions] Success, Attached to Creat
eProcessInternalW( )

warning: [9308] [CCreateProcessCtrl::AttachFunctions] Success, Attached to Creat
eThread( )

warning: [9308] [CCreateProcessCtrl::AttachFunctions] Success, Attach Function C
oGetObject

warning: [9308] [CCreateProcessCtrl::AttachFunctions] Success, Attached to Shell
ExecuteExW( )

warning: [9308] [CCreateProcessCtrl::AttachFunctions] Did NOT attach to CoGetCla
ssObjectFromURL. Function pointer is NULL.

warning: [9308] [CCreateProcessCtrl::AttachFunctions] Did NOT attach to Real_CoG
etClassObjectFromURLInternal. Function pointer is NULL.

warning: [9308] [APCFuncInstallHook] UrmHook::Install Result [0]

warning: [9308] [CDetourCreateProcess::Detoured_CreateThread] CreateThread paren
t[9308] new[9888]

[New Thread 9992.0x26a0]

Program received signal SIGSEGV, Segmentation fault.
0x77868e19 in ntdll!RtlIntegerToUnicodeString ()
   from C:\Windows\system32\ntdll.dll
(gdb) continue
Continuing.

Program received signal SIGSEGV, Segmentation fault.
0x77868e19 in ntdll!RtlIntegerToUnicodeString ()
   from C:\Windows\system32\ntdll.dll
(gdb) continue
Continuing.
[Inferior 1 (process 9992) exited with code 030000000005]
(gdb) continue
The program is not being run.
(gdb)
