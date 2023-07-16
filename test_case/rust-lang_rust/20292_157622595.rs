
(gdb) break debug_println.rs:2
Breakpoint 1 at 0x544b: file debug_println.rs, line 2.
(gdb) r
...

Breakpoint 1, debug_println::main () at debug_println.rs:2
2       let a = 42;
(gdb) n
3       println!("The answer = {}", a);
(gdb) n
2   <std macros>: No such file or directory.
(gdb) n
5   }
(gdb) n
0x0000555555561b55 in sys_common::unwind::try::try_fn::h4848098439110500489 ()
(gdb) 
(gdb)
(gdb) r
...

Breakpoint 1, debug_println::main () at debug_println.rs:2
2       let a = 42;
(gdb) s
3       println!("The answer = {}", a);
(gdb) s
2   <std macros>: No such file or directory.
(gdb) s
debug_println::fmt::ArgumentV1<'a>::new<i32> (x=0x7fffffffdb18, f=0x55555558a570 <fmt::num::i32.fmt..Display::fmt::h05897f40d7b1bb4buzU>) at ../src/libcore/fmt/mod.rs:192
192 ../src/libcore/fmt/mod.rs: No such file or directory.
(gdb) s
193 in ../src/libcore/fmt/mod.rs
(gdb) s
189 in ../src/libcore/fmt/mod.rs
(gdb) s
debug_println::fmt::Arguments<'a>::new_v1 (pieces=&[&str](len: 2) = {...}, args=&[core::fmt::ArgumentV1](len: 1) = {...}) at ../src/libcore/fmt/mod.rs:226
226 in ../src/libcore/fmt/mod.rs
(gdb) s
debug_println::fmt::Arguments<'a>::new_v1 (pieces=&[&str](len: 1) = {...}, args=&[core::fmt::ArgumentV1](len: 0)) at ../src/libcore/fmt/mod.rs:226
226 in ../src/libcore/fmt/mod.rs
(gdb) s
debug_println::main () at debug_println.rs:5
5   }
(gdb) s
0x0000555555561b55 in sys_common::unwind::try::try_fn::h4848098439110500489 ()
(gdb) 
