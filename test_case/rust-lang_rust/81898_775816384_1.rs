
$ lldb ./rust_out
(lldb) target create "./rust_out"
Current executable set to './rust_out' (x86_64).
(lldb) rbreak rust_out
Breakpoint 1: 2 locations.
(lldb) run
Process 4075 launched: './rust_out' (x86_64)
Process 4075 stopped
* thread #1, name = 'rust_out', stop reason = breakpoint 1.1
    frame #0: 0x000055555555932b rust_out`rust_out::main::h405f65324f7b53be at <anon>:1
(lldb) cont
Process 4075 resuming
Process 4075 stopped
* thread #1, name = 'rust_out', stop reason = breakpoint 1.2
    frame #0: 0x000055555555931a rust_out`rust_out::foo::ha632284b5a2f1ab8(a=(data_ptr = "a", length = 1)) at <anon>:1
(lldb)
