
$ gdb rustc.exe
GNU gdb (GDB) 7.5
...
This GDB was configured as "i686-pc-mingw32".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from c:\Program Files (x86)\Rust\bin\rustc.exe...(no debugging symbols found)...done.
(gdb) run
Starting program: c:\Program Files (x86)\Rust\bin\rustc.exe
[New Thread 5196.0x1068]

Program received signal SIGSEGV, Segmentation fault.
0x6fc5b93f in libstdc++-6!_ZN9__gnu_cxx18__exchange_and_addEPVii ()
   from C:\MinGW\bin\libstdc++-6.dll
