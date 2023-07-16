plain
failures:

---- [debuginfo-cdb] tests\debuginfo\type-names.rs stdout ----

error: line not found in debugger output: struct ref$<dyn$<type_names::Trait2<type_names::Struct1,type_names::Struct1> > > generic_ref_trait = [...]
status: exit code: 0
command: PATH="D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.9.0\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps" "C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x86\\cdb.exe" "-lines" "-cf" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\type-names.cdb\\type-names.debugger.script" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\debuginfo\\type-names.cdb\\a.exe"
Microsoft (R) Windows Debugger Version 10.0.22000.832 X86
Copyright (c) Microsoft Corporation. All rights reserved.


CommandLine: D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\type-names.cdb\a.exe

************* Path validation summary **************
Response                         Time (ms)     Location
Deferred                                       srv*
Symbol search path is: srv*
Executable search path is: 
ModLoad: 01260000 0126a000   a.exe   
ModLoad: 77d80000 77f1e000   ntdll.dll
ModLoad: 75f80000 76060000   C:\Windows\SysWOW64\KERNEL32.DLL
ModLoad: 77a10000 77c11000   C:\Windows\SysWOW64\KERNELBASE.dll
ModLoad: 72370000 7240c000   C:\Windows\SysWOW64\apphelp.dll
ModLoad: 75cc0000 75de3000   C:\Windows\SysWOW64\ucrtbase.dll
ModLoad: 72270000 72285000   C:\Windows\SysWOW64\VCRUNTIME140.dll
ModLoad: 693a0000 69912000   D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-3dc484373ef2641b.dll
ModLoad: 77c20000 77ca0000   C:\Windows\SysWOW64\ADVAPI32.dll
ModLoad: 754d0000 75590000   C:\Windows\SysWOW64\msvcrt.dll
ModLoad: 75bf0000 75c69000   C:\Windows\SysWOW64\sechost.dll
ModLoad: 768c0000 7697d000   C:\Windows\SysWOW64\RPCRT4.dll
ModLoad: 753b0000 753d0000   C:\Windows\SysWOW64\SspiCli.dll
ModLoad: 753a0000 753aa000   C:\Windows\SysWOW64\CRYPTBASE.dll
ModLoad: 753d0000 75435000   C:\Windows\SysWOW64\bcryptPrimitives.dll
ModLoad: 755a0000 755ff000   C:\Windows\SysWOW64\WS2_32.dll
ModLoad: 77670000 77689000   C:\Windows\SysWOW64\bcrypt.dll
ModLoad: 72290000 722b5000   C:\Windows\SysWOW64\USERENV.dll
ModLoad: 76290000 762ac000   C:\Windows\SysWOW64\profapi.dll
(14bc.3d8): Break instruction exception - code 80000003 (first chance)
eax=00000000 ebx=00000010 ecx=dfbf0000 edx=00000000 esi=010eb000 edi=77d8688c
eip=77e2f886 esp=0136f3b8 ebp=0136f3e4 iopl=0         nv up ei pl zr na pe nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000246
ntdll!LdrpDoDebuggerBreak+0x2b:
77e2f886 cc              int     3
0:000> version
Windows 10 Version 17763 MP (8 procs) Free x86 compatible
Product: Server, suite: TerminalServer DataCenter SingleUserTS
Edition build lab: 17763.1.x86fre.rs5_release.180914-1434
Build layer:            -> 
Build layer:            -> 
Build layer:            -> 
Machine Name:
Debug session time: Tue Feb 28 10:51:50.753 2023 (UTC + 0:00)
System Uptime: 0 days 5:38:40.628
Process Uptime: 0 days 0:00:00.076
  Kernel time: 0 days 0:00:00.000
  User time: 0 days 0:00:00.000
Live user mode: <Local>
Microsoft (R) Windows Debugger Version 10.0.22000.832 X86
Copyright (c) Microsoft Corporation. All rights reserved.


command line: '"C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\cdb.exe" -lines -cf D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\type-names.cdb\type-names.debugger.script D:\a\rust\rust\build\i686-pc-windows-msvc\test\debuginfo\type-names.cdb\a.exe'  Debugger Process 0x5E0 
dbgeng:  image 10.0.22000.832, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbgeng.dll]
dbghelp: image 10.0.22000.832, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
        DIA version: 29395
Extension DLL search Path:
    C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\arcade;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\pri;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;C:\Users\runneradmin\AppData\Local\Dbg\EngineExtensions32;C:\Program Files (x86)\Windows Kits\10\Debuggers\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x64;C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\14.29.30133\bin\HostX64\x86;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0-bootstrap-tools\i686-pc-windows-msvc\release\deps;D:\a\rust\rust\build\i686-pc-windows-msvc\stage0\bin;D:\a\rust\rust\ninja;D:\a\rust\rust\msys2\mingw32\bin;C:\hostedtoolcache\windows\Python\3.11.2\x64\Scripts;C:\hostedtoolcache\windows\Python\3.11.2\x64;C:\msys64\usr\bin;D:\a\rust\rust\sccache;C:\Program Files\MongoDB\Server\5.0\bin;C:\aliyun-cli;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\tools\zstd;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.9.3\x64;C:\cabal\bin;C:\ghcup\bin;C:\Program Files\dotnet;C:\mysql\bin;C:\Program Files\R\R-4.2.2\bin\x64;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\Program Files (x86)\pipx_bin;C:\npm\prefix;C:\hostedtoolcache\windows\go\1.17.13\x64\bin;C:\hostedtoolcache\windows\Python\3.7.9\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.9\x64;C:\hostedtoolcache\windows\Ruby\2.5.9\x64\bin;C:\tools\kotlinc\bin;C:\hostedtoolcache\windows\Java_Temurin-Hotspot_jdk\8.0.362-9\x64\bin;C:\Program Files\ImageMagick-7.1.0-Q16-HDRI;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\ProgramData\kind;C:\Program Files\Eclipse Foundation\jdk-8.0.302.8-hotspot\bin;C:\Windows\system32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0;C:\Windows\System32\OpenSSH;C:\ProgramData\Chocolatey\bin;C:\Program Files\PowerShell\7;C:\Program Files\Microsoft\Web Platform Installer;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files\Microsoft SQL Server\Client SDK\ODBC\170\Tools\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\160\DTS\Binn;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\ProgramData\chocolatey\lib\pulumi\tools\Pulumi\bin;C:\Program Files\TortoiseSVN\bin;C:\Program Files\CMake\bin;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.9.0\bin;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\Program Files\GitHub CLI;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\SessionManagerPlugin\bin;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Program Files (x86)\Google\Cloud SDK\google-cloud-sdk\bin;C:\Program Files (x86)\Microsoft BizTalk Server;C:\Program Files\LLVM\bin;C:\Users\runneradmin\.dotnet\tools;C:\Users\runneradmin\.cargo\bin;C:\Users\runneradmin\AppData\Local\Microsoft\WindowsApps
Extension DLL chain:
    dbghelp: image 10.0.22000.832, API 10.0.6, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\dbghelp.dll]
    ext: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\ext.dll]
    wow64exts: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\wow64exts.dll]
    exts: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\exts.dll]
Some tests failed in compiletest suite=debuginfo mode=debuginfo host=i686-pc-windows-msvc target=i686-pc-windows-msvc
    uext: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\winext\uext.dll]
    ntsdexts: image 10.0.22000.832, API 1.0.0, 
        [path: C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\WINXP\ntsdexts.dll]
WOW64 extensions loaded
0:000> .nvlist
Loaded NatVis Files:
    <None Loaded>
0:000> .scriptload "D:\a\rust\rust\tests\debuginfo\type-names.cdb.js"
JavaScript script successfully loaded from 'D:\a\rust\rust\tests\debuginfo\type-names.cdb.js'
0:000> bp `type-names.rs:467`
*** WARNING: Unable to verify checksum for a.exe
0:000>  g
Breakpoint 0 hit
eax=0136f6c4 ebx=010eb000 ecx=00000004 edx=001d3cb0 esi=012664f4 edi=0136f89c
eip=0126469a esp=0136f568 ebp=0136f7b4 iopl=0         nv up ei pl nz na po nc
cs=0023  ss=002b  ds=002b  es=002b  fs=0053  gs=002b             efl=00000202
a!type_names::main+0x63a:
0126469a c745f00b000000  mov     dword ptr [ebp-10h],0Bh ss:002b:0136f7a4=0000000a
0:000> dv /t *_struct
struct type_names::GenericStruct<enum2$<type_names::mod1::Enum2>,f64> mut_generic_struct = struct type_names::GenericStruct<enum2$<type_names::mod1::Enum2>,f64>
0:000> dv /t *_enum_*
union enum2$<type_names::Enum1> simple_enum_1 = Variant1
union enum2$<type_names::Enum1> simple_enum_2 = union enum2$<type_names::Enum1>
union enum2$<type_names::mod1::Enum2> simple_enum_3 = union enum2$<type_names::mod1::Enum2>
union enum2$<type_names::mod1::mod2::Enum3<type_names::mod1::Struct2> > generic_enum_1 = Variant1
union enum2$<type_names::mod1::mod2::Enum3<type_names::Struct1> > generic_enum_2 = union enum2$<type_names::mod1::mod2::Enum3<type_names::Struct1> >
0:000> dv /t tuple*
struct tuple$<u32,type_names::Struct1,enum2$<type_names::mod1::mod2::Enum3<type_names::mod1::Struct2> > > tuple1 = (0x8, {...}, {...})
struct tuple$<tuple$<type_names::Struct1,type_names::mod1::mod2::Struct3>,enum2$<type_names::mod1::Enum2>,char> tuple2 = (({...}, {...}), Variant1, 0x78 'x')
0:000> dv /t box*
struct tuple$<alloc::boxed::Box<f32,alloc::alloc::Global>,i32> box1 = (1.000000, 0)
struct tuple$<alloc::boxed::Box<enum2$<type_names::mod1::mod2::Enum3<f32> >,alloc::alloc::Global>,i32> box2 = ({...}, 0)
struct alloc::boxed::Box<dyn$<type_names::Trait1>,alloc::alloc::Global> box_trait = struct alloc::boxed::Box<dyn$<type_names::Trait1>,alloc::alloc::Global>
0:000> dv /t *ref*
struct tuple$<ref$<type_names::Struct1>,i32> ref1 = ({...}, 0)
struct tuple$<ref$<type_names::GenericStruct<char,type_names::Struct1> >,i32> ref2 = ({...}, 0)
struct tuple$<ref_mut$<type_names::Struct1>,i32> mut_ref1 = ({...}, 0)
struct tuple$<ref_mut$<type_names::GenericStruct<enum2$<type_names::mod1::Enum2>,f64> >,i32> mut_ref2 = ({...}, 0)
struct ref$<dyn$<type_names::Trait1> > ref_trait = struct ref$<dyn$<type_names::Trait1> >
struct ref_mut$<dyn$<type_names::Trait1> > mut_ref_trait = struct ref_mut$<dyn$<type_names::Trait1> >
struct ref$<dyn$<type_names::Trait2<type_names::Struct1,type_names::Struct1> > > generic_ref_trait = struct ref$<dyn$<type_names::Trait2<type_names::Struct1,type_names::Struct1> > >
int generic_mut_ref_trait_impl = 0n0
struct ref_mut$<dyn$<type_names::Trait2<type_names::mod1::mod2::Struct3,type_names::GenericStruct<usize,isize> > > > generic_mut_ref_trait = struct ref_mut$<dyn$<type_names::Trait2<type_names::mod1::mod2::Struct3,type_names::GenericStruct<usize,isize> > > >
0:000> dv /t *_ptr*
struct tuple$<ptr_mut$<type_names::Struct1>,isize> mut_ptr1 = ({...}, 0)
struct tuple$<ptr_mut$<isize>,isize> mut_ptr2 = struct tuple$<ptr_mut$<isize>,isize>
struct tuple$<ptr_mut$<enum2$<type_names::mod1::mod2::Enum3<type_names::Struct1> > >,isize> mut_ptr3 = ({...}, 0)
struct tuple$<ptr_const$<type_names::Struct1>,isize> const_ptr1 = ({...}, 0)
struct tuple$<ptr_const$<isize>,isize> const_ptr2 = struct tuple$<ptr_const$<isize>,isize>
struct tuple$<ptr_const$<enum2$<type_names::mod1::mod2::Enum3<type_names::Struct1> > >,isize> const_ptr3 = ({...}, 0)
0:000> dv /t *vec*
struct tuple$<array$<type_names::Struct1,3>,i16> fixed_size_vec1 = ({...}, 0)
struct tuple$<array$<usize,3>,i16> fixed_size_vec2 = ({...}, 0)
struct alloc::vec::Vec<usize,alloc::alloc::Global> vec1 = { len=0x3 }
struct alloc::vec::Vec<enum2$<type_names::mod1::Enum2>,alloc::alloc::Global> vec2 = { len=0x1 }
0:000> dv /t slice*
struct ref$<slice2$<usize> > slice1 = { len=0x3 }
struct ref_mut$<slice2$<enum2$<type_names::mod1::Enum2> > > slice2 = { len=0x1 }
0:000> dv /t *_trait
struct alloc::boxed::Box<dyn$<type_names::Trait1>,alloc::alloc::Global> box_trait = struct alloc::boxed::Box<dyn$<type_names::Trait1>,alloc::alloc::Global>
struct ref$<dyn$<type_names::Trait1> > ref_trait = struct ref$<dyn$<type_names::Trait1> >
struct ref_mut$<dyn$<type_names::Trait1> > mut_ref_trait = struct ref_mut$<dyn$<type_names::Trait1> >
struct alloc::boxed::Box<dyn$<core::marker::Send,core::marker::Sync>,alloc::alloc::Global> no_principal_trait = struct alloc::boxed::Box<dyn$<core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
struct ref$<dyn$<type_names::Trait3<u32,assoc$<AssocType,isize> >,core::marker::Send> > has_associated_type_trait = struct ref$<dyn$<type_names::Trait3<u32,assoc$<AssocType,isize> >,core::marker::Send> >
struct ref$<dyn$<type_names::TraitNoGenericsButWithAssocType<assoc$<Output,isize> > > > has_associated_type_but_no_generics_trait = struct ref$<dyn$<type_names::TraitNoGenericsButWithAssocType<assoc$<Output,isize> > > >
struct alloc::boxed::Box<dyn$<type_names::Trait2<i32,type_names::mod1::Struct2> >,alloc::alloc::Global> generic_box_trait = struct alloc::boxed::Box<dyn$<type_names::Trait2<i32,type_names::mod1::Struct2> >,alloc::alloc::Global>
struct ref$<dyn$<type_names::Trait2<type_names::Struct1,type_names::Struct1> > > generic_ref_trait = struct ref$<dyn$<type_names::Trait2<type_names::Struct1,type_names::Struct1> > >
struct ref_mut$<dyn$<type_names::Trait2<type_names::mod1::mod2::Struct3,type_names::GenericStruct<usize,isize> > > > generic_mut_ref_trait = struct ref_mut$<dyn$<type_names::Trait2<type_names::mod1::mod2::Struct3,type_names::GenericStruct<usize,isize> > > >
0:000> dv /t *_fn*
struct tuple$<void (*)(enum2$<core::option::Option<isize> >,enum2$<core::option::Option<ref$<type_names::mod1::Struct2> > >),usize> rust_fn = ({...}, 0x0)
struct tuple$<void (*)(isize),usize> extern_c_fn = ({...}, 0x0)
struct tuple$<void (*)(enum2$<core::result::Result<char,f64> >),usize> unsafe_fn = ({...}, 0x0)
struct tuple$<usize (*)(f64),usize> rust_fn_with_return_value = ({...}, 0x0)
struct tuple$<type_names::Struct1 (*)(),usize> extern_c_fn_with_return_value = ({...}, 0x0)
struct tuple$<type_names::mod1::Struct2 (*)(type_names::GenericStruct<u16,u8>),usize> unsafe_fn_with_return_value = ({...}, 0x0)
0:000> dv /t *_function*
struct tuple$<isize (*)(isize),usize> generic_function_int = ({...}, 0x0)
struct tuple$<type_names::mod1::mod2::Struct3 (*)(type_names::mod1::mod2::Struct3),usize> generic_function_struct3 = ({...}, 0x0)
struct tuple$<isize (*)(ptr_const$<u8>, ...),usize> variadic_function = ({...}, 0x0)
0:000> dx Debugger.State.Scripts.@"type-names.cdb".Contents.getFunctionDetails("rust_fn")
*** WARNING: Unable to verify checksum for D:\a\rust\rust\build\i686-pc-windows-msvc\stage2\lib\rustlib\i686-pc-windows-msvc\lib\std-3dc484373ef2641b.dll
Return Type: void
Parameter Types: enum2$<core::option::Option<isize> >,enum2$<core::option::Option<ref$<type_names::mod1::Struct2> > >
Debugger.State.Scripts.@"type-names.cdb".Contents.getFunctionDetails("rust_fn")
0:000> dx Debugger.State.Scripts.@"type-names.cdb".Contents.getFunctionDetails("rust_fn_with_return_value")
Return Type: usize
Parameter Types: f64
Debugger.State.Scripts.@"type-names.cdb".Contents.getFunctionDetails("rust_fn_with_return_value")
0:000> dx Debugger.State.Scripts.@"type-names.cdb".Contents.getFunctionDetails("extern_c_fn_with_return_value")
Return Type: type_names::Struct1
Parameter Types: 
Debugger.State.Scripts.@"type-names.cdb".Contents.getFunctionDetails("extern_c_fn_with_return_value")
0:000> dv /t closure*
struct tuple$<type_names::main::closure_env$0,usize> closure1 = ({...}, 0x0)
struct tuple$<type_names::main::closure_env$1,usize> closure2 = ({...}, 0x0)
0:000> dv /t foreign*
struct type_names::extern$0::ForeignType1 * foreign1 = 0x00000000
struct type_names::mod1::extern$0::ForeignType2 * foreign2 = 0x00000000
0:000> qq
quit:
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\atlmfc.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\ObjectiveC.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\concurrency.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\cpp_rest.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\stl.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Data.Json.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Geolocation.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Devices.Sensors.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\Windows.Media.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\windows.natvis'
NatVis script unloaded from 'C:\Program Files (x86)\Windows Kits\10\Debuggers\x86\Visualizers\winrt.natvis'
JavaScript script unloaded from 'D:\a\rust\rust\tests\debuginfo\type-names.cdb.js'
stderr: none




failures:
    [debuginfo-cdb] tests\debuginfo\type-names.rs

test result: FAILED. 127 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 19.71s

Build completed unsuccessfully in 0:29:24
make: *** [Makefile:68: ci-subset-1] Error 1
