
Z:\dllimport> @'
>> fn main() {
>>     println!("Hello World");
>> }
>> '@ > dllimport.rs
Z:\dllimport> rustc dllimport.rs
Z:\dllimport> dumpbin /disasm dllimport.exe | Select-String WriteConsoleW

  000000014000E395: E8 69 D3 00 00     call        WriteConsoleW
  000000014000E3F6: E8 08 D3 00 00     call        WriteConsoleW
WriteConsoleW:
  000000014001B703: FF 25 2F 09 00 00  jmp         qword ptr [__imp_WriteConsoleW]

Z:\dllimport> rustc +dllimport dllimport.rs
Z:\dllimport> dumpbin /disasm dllimport.exe | Select-String WriteConsoleW

  00000001400027BF: FF 15 93 88 01 00  call        qword ptr [__imp_WriteConsoleW]
  0000000140002855: FF 15 FD 87 01 00  call        qword ptr [__imp_WriteConsoleW]

