
---- [debuginfo-both] debuginfo\gdb-pretty-struct-and-enums.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7010001
error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}

...

GNU gdb (GDB) 7.10.1
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "i686-w64-mingw32".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x401632: file C:\projects\rust\src/test\debuginfo\gdb-pretty-struct-and-enums.rs, line 68.
[New Thread 5156.0x17ec]
[New Thread 5156.0xa34]
[New Thread 5156.0x348]
[New Thread 5156.0x1758]

Breakpoint 1, gdb_pretty_struct_and_enums::main::h44925421ce933524 () at C:\projects\rust\src/test\debuginfo\gdb-pretty-struct-and-enums.rs:68
68	    zzz(); // #break
$1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
$2 = {<No data fields>}
$3 = CStyleEnumVar1
$4 = CStyleEnumVar2
$5 = CStyleEnumVar3
A debugging session is active.

	Inferior 1 [process 5156] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]

------------------------------------------
stderr:
------------------------------------------
Warning: C:projectsrust./src/etc: No such file or directory.
