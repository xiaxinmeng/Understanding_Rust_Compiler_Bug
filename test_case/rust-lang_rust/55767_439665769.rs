
[01:39:08] ---- [debuginfo-both] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
[01:39:08] NOTE: compiletest thinks it is using GDB without native rust support
[01:39:08] 
[01:39:08] error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:39:08] status: exit code: 0
[01:39:08] command: "arm-linux-androideabi-gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/gdb-pretty-struct-and-enums.debugger.script"
[01:39:08] stdout:
[01:39:08] ------------------------------------------
[01:39:08] 0xb6efda30 in ?? ()
[01:39:08] Breakpoint 1 at 0xb6f0f7ac: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 69.
[01:39:08] 
[01:39:08] Breakpoint 1, gdb_pretty_struct_and_enums::main::h44925421ce933524 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:69
[01:39:08] 69	    zzz(); // #break
[01:39:08] $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
[01:39:08] $2 = {<No data fields>}
[01:39:08] $3 = CStyleEnumVar1
[01:39:08] $4 = CStyleEnumVar2
[01:39:08] $5 = CStyleEnumVar3
[01:39:08] A debugging session is active.
[01:39:08] 
[01:39:08] 	Inferior 1 [process 10377] will be killed.
[01:39:08] 
[01:39:08] Quit anyway? (y or n) [answered Y; input not from terminal]
[01:39:08] 
[01:39:08] ------------------------------------------
[01:39:08] stderr:
[01:39:08] ------------------------------------------
[01:39:08] warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
[01:39:08] of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/a.
[01:39:08] Use `info auto-load python-scripts [REGEXP]' to list them.
[01:39:08] warning: Unable to find dynamic linker breakpoint function.
[01:39:08] GDB will be unable to debug shared library initializers
[01:39:08] and track explicitly loaded dynamic code.
[01:39:08] warning: Unable to find dynamic linker breakpoint function.
[01:39:08] GDB will be unable to debug shared library initializers
[01:39:08] and track explicitly loaded dynamic code.
[01:39:08] warning: Could not load shared library symbols for 4 libraries, e.g. /system/bin/linker.
[01:39:08] Use the "info sharedlibrary" command to see the complete listing.
[01:39:08] Do you need "set solib-search-path" or "set sysroot"?
[01:39:08] 
[01:39:08] ------------------------------------------
