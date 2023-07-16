plain
test [debuginfo-gdb] src/test/debuginfo/vec.rs ... ignored

failures:

---- [debuginfo-gdb] src/test/debuginfo/thread-names.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support

error: line not found in debugger output:   1    Thread [...] [...] "main" [...]
status: exit status: 0
command: "arm-linux-androideabi-gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/thread-names.gdb/thread-names.debugger.script"
--- stdout -------------------------------
0xb6f4fa30 in ?? ()
Breakpoint 1 at 0xb6f64be8: file /checkout/src/test/debuginfo/thread-names.rs, line 34.
[New Thread 24223.24224]
[Switching to Thread 24223.24224]

Thread 2 hit Breakpoint 1, thread_names::main::_$u7b$$u7b$closure$u7d$$u7d$::hd5f69bd5cd9d8878 () at /checkout/src/test/debuginfo/thread-names.rs:34
34         zzz(); // #break
  Id   Target Id         Frame 
  1    Thread 24223.24223 _$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$::branch::hb8a5415d4e0928b9 (self=...) at /checkout/library/core/src/result.rs:2110
* 2    Thread 24223.24224 thread_names::main::_$u7b$$u7b$closure$u7d$$u7d$::hd5f69bd5cd9d8878 () at /checkout/src/test/debuginfo/thread-names.rs:34
A debugging session is active.

 Inferior 1 [process 24223] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
--- stderr -------------------------------
--- stderr -------------------------------
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/thread-names.gdb/a.
Use `info auto-load python-scripts [REGEXP]' to list them.
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
warning: Could not load shared library symbols for 4 libraries, e.g. /system/bin/linker.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?



failures:
