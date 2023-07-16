
rustc -Z extra-debug-info prog1.rs 
asamoilov@atlant:~/work/projects/rust-sandbox/my-rust$ gdb prog1 
GNU gdb (GDB) 7.6.1
Copyright (C) 2013 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-unknown-linux-gnu".
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>...
Reading symbols from /home/asamoilov/work/projects/rust-sandbox/my-rust/prog1...done.
(gdb) l
1   // **********************************************
2   //prog1.rs <http://prog1.rs>
3   use std::rand::{task_rng, Rng};
4   fn main() {
5       let names = ["Alice", "Bob", "Carol"];
6       for name in names.iter() {
7           let v = task_rng().shuffle(~[1,2,3]);
8           for num in v.iter() {
9               println!("{:s} says: {:d}", *name, *num);
10          }
(gdb) b 4
Breakpoint 1 at 0x406340: file prog1.rs, line 4.
(gdb) n
The program is not being run.
(gdb) r
Starting program: /home/asamoilov/work/projects/rust-sandbox/my-rust/prog1 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[New Thread 0x7ffff7f66700 (LWP 8881)]
[New Thread 0x7ffff6edf700 (LWP 8882)]
[New Thread 0x7fffeffff700 (LWP 8883)]
[New Thread 0x7ffff6dde700 (LWP 8884)]
[New Thread 0x7ffff6cdd700 (LWP 8885)]
[New Thread 0x7ffff6bdc700 (LWP 8886)]
[New Thread 0x7ffff6adb700 (LWP 8887)]
[New Thread 0x7ffff69da700 (LWP 8888)]
[Switching to Thread 0x7ffff7f66700 (LWP 8881)]

Breakpoint 1, prog1::main () at prog1.rs:4
4   fn main() {
(gdb) n
5       let names = ["Alice", "Bob", "Carol"];
(gdb) n
6       for name in names.iter() {
(gdb) p names
$1 = {{data_ptr = 0x52f5b0 <str1489> "Alice", length = 5}, {data_ptr = 0x52f5b6 <str1490> "Bob", length = 3}, {
    data_ptr = 0x52f5ba <str1491> "Carol", length = 5}}
(gdb) 
