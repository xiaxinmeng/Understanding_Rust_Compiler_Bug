
herugrim::~/code/limerick $ rustc -g bug.rs      
bug.rs:12:0: 34:1 warning: unused variable args
bug.rs:12 fn main(args: [str])
bug.rs:13 {
bug.rs:14     fn nover<T: copy, U: copy>(m: map::hashmap<T, U>, k: T, v: U) {
bug.rs:15         if ! m.insert(k, v) {
bug.rs:16             fail;
bug.rs:17         }
          ...
herugrim::~/code/limerick $ gdb ./bug      
GNU gdb 6.3.50-20050815 (Apple version gdb-1752) (Sat Jan 28 03:02:46 UTC 2012)
Copyright 2004 Free Software Foundation, Inc.
GDB is free software, covered by the GNU General Public License, and you are
welcome to change it and/or distribute copies of it under certain conditions.
Type "show copying" to see the conditions.
There is absolutely no warranty for GDB.  Type "show warranty" for details.
This GDB was configured as "x86_64-apple-darwin"...
Reading symbols for shared libraries .. done

(gdb) b malloc_error_break
Function "malloc_error_break" not defined.
Make breakpoint pending on future shared library load? (y or [n]) y

Breakpoint 1 (malloc_error_break) pending.
(gdb) run
Starting program: /Users/grahame/code/limerick/bug 
Reading symbols for shared libraries ..+................................................................. done
Breakpoint 1 at 0x7fff97c6b6c0
Pending breakpoint 1 - "malloc_error_break" resolved
bug(2167,0x10061a000) malloc: *** error for object 0x1004146b1: pointer being freed was not allocated
*** set a breakpoint in malloc_error_break to debug
[Switching to process 2167 thread 0x1a03]

Breakpoint 1, 0x00007fff97c6b6c0 in malloc_error_break ()
(gdb) bt
#0  0x00007fff97c6b6c0 in malloc_error_break ()
#1  0x00007fff97c6b805 in free ()
#2  0x000000010036ab06 in __morestack () at rust_task.cpp:1327
#3  0x000000010035b5fe in rust_task::call_on_c_stack (this=0x10100a000, 
    args=0x10100f730, fn_ptr=0x10035d0b0) at rust_task.h:352
#4  0x000000010035d088 in upcall_shared_free (ptr=0x1007dfeb8)
    at rust_upcall.cpp:50
#5  0x0000000100003617 in glue_free80 ()
#6  0x0000000100003780 in glue_drop118 ()
#7  0x0000000100006875 in glue_drop542 ()
#8  0x0000000100006af7 in glue_free567 ()
#9  0x0000000100006a63 in f2 () at bug.rs:28
#10 0x00000001000011d7 in main () at bug.rs:33
#11 0x0000000100001292 in _rust_main ()
#12 0x000000010035b0d5 in task_start_wrapper (a=0x1004146b1)
    at rust_task.cpp:141
