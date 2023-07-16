
[root@s7netserver dev]# gdb -quiet --args rustc --version
Reading symbols from rustc...(no debugging symbols found)...done.
(gdb) rbreak .
	<output deleted>
(gdb) commands
Type commands for breakpoint(s) 1-31, one per line.
End with a line saying just "end".
>continue
>end
(gdb) run
Starting program: /usr/local/bin/rustc --version
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/libthread_db.so.1".

	<output deleted>

---Type <return> to continue, or q <return> to quit---
Breakpoint 4, 0x800007a0 in __rde_alloc@plt ()

Breakpoint 24, 0x80000a50 in __rust_alloc ()

Breakpoint 4, 0x800007a0 in __rde_alloc@plt ()

Breakpoint 24, 0x80000a50 in __rust_alloc ()

Breakpoint 4, 0x800007a0 in __rde_alloc@plt ()

Breakpoint 24, 0x80000a50 in __rust_alloc ()

Breakpoint 4, 0x800007a0 in __rde_alloc@plt ()

Breakpoint 24, 0x80000a50 in __rust_alloc ()

Breakpoint 4, 0x800007a0 in __rde_alloc@plt ()

Breakpoint 26, 0x80000ab0 in __rust_realloc ()

Breakpoint 6, 0x800007c0 in __rde_realloc@plt ()

Program terminated with signal SIGSEGV, Segmentation fault.
The program no longer exists.
(gdb) 
