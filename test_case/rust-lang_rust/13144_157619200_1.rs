
(gdb) break debug_fn_static.rs:4
Breakpoint 1 at 0x5164: file debug_fn_static.rs, line 4.
(gdb) run
...

Breakpoint 1, debug_fn_static::main () at debug_fn_static.rs:4
4   }
(gdb) info variables .*SIDE.*
All variables matching regular expression ".*SIDE.*":
(gdb) 
