
(gdb) b 4
Breakpoint 1 at 0x406340: file prog1.rs, line 4.
(gdb) r
Starting program: /home/asamoilov/work/projects/rust-sandbox/my-rust/prog1 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7ffff7f66700 (LWP 8790)]
...
[Switching to Thread 0x7ffff7f66700 (LWP 8790)]

Breakpoint 1, prog1::main () at prog1.rs:4
4   fn main() {
(gdb) n
5       let names = ["Alice", "Bob", "Carol"];
(gdb) n
6       for name in names.iter() {
(gdb) p names
No symbol "names" in current context.
(gdb) info scope 4
Scope for 4 contains no locals or arguments.
(gdb) info scope main
Scope for main contains no locals or arguments.
