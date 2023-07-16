
Program received signal SIGSEGV, Segmentation fault.
0x0000555555568e15 in hello::expensive_computation () at src/main.rs:23
23	       *(0xaabbccdd as *mut usize) = 1234;
(gdb) bt
#0  0x0000555555568e15 in hello::expensive_computation () at src/main.rs:23
#1  0x0000555555568dc6 in hello::get_cached_val::{closure#0} () at src/main.rs:15
#2  0x0000555555568db1 in std::sync::once::{impl#2}::call_once::{closure#0}<hello::get_cached_val::{closure_env#0}> ()
    at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sync/once.rs:149
#3  0x0000555555568b77 in std::sys_common::once::futex::Once::call<std::sync::once::{impl#2}::call_once::{closure_env#0}<hello::get_cached_val::{closure_env#0}>> (
    self=0x5555555a5340 <hello::INIT::h9c84dac174d4d2e5>, ignore_poisoning=false, f=0x7fffffffdb28)
    at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sys_common/once/futex.rs:124
#4  0x0000555555568d16 in std::sync::once::Once::call_once<hello::get_cached_val::{closure_env#0}> (self=0x5555555a5340 <hello::INIT::h9c84dac174d4d2e5>, f=...)
    at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sync/once.rs:149
#5  0x0000555555568e04 in hello::get_cached_val () at src/main.rs:14
#6  0x0000555555568e36 in hello::main () at src/main.rs:29
(gdb) f 2
#2  0x0000555555568db1 in std::sync::once::{impl#2}::call_once::{closure#0}<hello::get_cached_val::{closure_env#0}> ()
    at /rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sync/once.rs:149
149	/rustc/b955c8271da80a1af8a1d54c4e1bbdaf51b032e9/library/std/src/sync/once.rs: No such file or directory.
