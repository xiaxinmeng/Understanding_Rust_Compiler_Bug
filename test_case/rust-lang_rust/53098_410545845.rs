
---- [debuginfo-gdb] debuginfo\borrowed-c-style-enum.rs stdout ----
NOTE: compiletest thinks it is using GDB with native rust support
NOTE: compiletest thinks it is using GDB version 7012001

error: line not found in debugger output: $1 = borrowed_c_style_enum::ABC::TheA
status: exit code: 0
command: PATH="C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage1\lib\rustlib\x86_64-pc-windows-gnu\lib;C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\llvm\build\bin;C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage0-bootstrap-tools\x86_64-pc-windows-gnu\release\deps;C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage0\bin;C:\msys64\mingw64\bin;C:\Users\we\.cargo\bin;C:\Program Files\CMake\bin;C:\Program Files\Git\cmd;C:\mingw-w64\x86_64-7.2.0-posix-seh-rt_v5-rev1\mingw64\bin;C:\ProgramData\chocolatey\bin;C:\msys64\mingw64\bin;C:\msys64\usr\local\bin;C:\msys64\usr\bin;C:\msys64\usr\bin;C:\Windows\System32;C:\Windows;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\msys64\usr\bin\site_perl;C:\msys64\usr\bin\vendor_perl;C:\msys64\usr\bin\core_perl" "C:\\msys64\\usr\\bin\\gdb" "-quiet" "-batch" "-nx" "-command=C:\\msys64\\home\\we\\rust\\build\\x86_64-pc-windows-gnu\\test\\debuginfo\\borrowed-c-style-enum\\borrowed-c-style-enum.debugger.script"
stdout:
------------------------------------------
GNU gdb (GDB) 7.12.1
Copyright (C) 2017 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-msys".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x401581: file C:\msys64\home\we\rust\src/test\debuginfo\borrowed-c-style-enum.rs, line 61.
[New Thread 6384.0x19f8]

Breakpoint 1, borrowed_c_style_enum::main::h533312d9b0c462da () at C:\msys64\home\we\rust\src/test\debuginfo\borrowed-c-style-enum.rs:61
61          zzz(); // #break
$1 = borrowed_c_style_enum::TheA
$2 = borrowed_c_style_enum::TheB
$3 = borrowed_c_style_enum::TheC
A debugging session is active.

        Inferior 1 [process 6384] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
Warning: /home/we/rust/C: No such file or directory.
Warning: /home/we/rust/msys64homewerust./src/etc: No such file or directory.
warning: SHIMVIEW: ShimInfo(Complete)

------------------------------------------
