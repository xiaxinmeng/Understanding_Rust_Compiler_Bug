
(gdb) ptype TEST
No symbol 'TEST' in current context
(gdb) set lang c
(gdb) ptype TEST
Reading in symbols for /home/m4b/tmp/bad_debug/nomangle.rs...done.
type = u64
(gdb) whatis TEST
type = u64
(gdb) info addr TEST
Symbol "TEST" is static storage at address 0x64940.
(gdb) set lang c++
(gdb) info addr TEST
Symbol "TEST" is static storage at address 0x64940.
(gdb) ptype TEST
type = u64
(gdb) break nomangle::main
Breakpoint 1 at 0x63c6: file /home/m4b/tmp/bad_debug/nomangle.rs, line 5.
(gdb) r
Starting program: /tmp/nomangle 
Using PIE (Position Independent Executable) displacement 0x555555554000 for "/tmp/nomangle".
..
Breakpoint 1, nomangle::main () at /home/m4b/tmp/bad_debug/nomangle.rs:5
5	    println!("{}", TEST);
Warning: the current language does not match this frame.
(gdb) ptype TEST
type = u64
(gdb) info addr TEST
Symbol "TEST" is static storage at address 0x5555555b8940.
(gdb) p TEST
$1 = 3735928559
(gdb) set lang rust
(gdb) p TEST
No symbol 'TEST' in current context
(gdb) info addr TEST
Symbol "TEST" is at 0x5555555b8940 in a file compiled without debugging.
(gdb) 
