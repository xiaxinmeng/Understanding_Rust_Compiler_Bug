
C:\local\htd>cargo +nightly build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
C:\local\htd>"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb" target\debug\htd.exe
[...]
0:000> bp `src\main.rs:4`
WARNING: Line information loading disabled
*** WARNING: Unable to verify checksum for htd.exe
0:000> g
ModLoad: 00007ffb`b99a0000 00007ffb`b9a1f000   C:\WINDOWS\System32\bcryptPrimitives.dll
Breakpoint 0 hit
htd!htd::main+0x3e:
00007ff7`eac97ade e8bd000000      call    htd!htd::zzz (00007ff7`eac97ba0)
0:000> dx hmap
hmap             : { size=0x1 } [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [<Raw View>]     [Type: std::collections::hash::map::HashMap<u64, u64, std::collections::hash::map::RandomState>]
    [size]           : 0x1 [Type: unsigned __int64]
    [capacity]       : 0x3
    [state]          [Type: std::collections::hash::map::RandomState]
    ["0x1"]          : 0x2 [Type: unsigned __int64]
0:000>
