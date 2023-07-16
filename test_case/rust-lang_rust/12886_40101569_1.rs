
(gdb) b inline.rs:2
Breakpoint 1 at 0x404ebb: inline.rs:2. (2 locations)
(gdb) r
Starting program: /tmp/inline 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib64/libthread_db.so.1".

Breakpoint 1, inline::bar () at inline.rs:2
2       let s = Some(5).unwrap();
(gdb) n

Breakpoint 1, option::Option$LT$T$GT$::unwrap::he0fbfb416e1da23bIaa::v0.0 () at inline.rs:2
2       let s = Some(5).unwrap();
(gdb) n
inline::bar () at inline.rs:3
3       s
(gdb) q
