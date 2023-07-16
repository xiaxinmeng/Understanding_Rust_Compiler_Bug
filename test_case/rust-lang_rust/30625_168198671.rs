
(gdb) break rust_panic
Breakpoint 1 at 0x43e04a
(gdb) run
Starting program: c:\Users\Jonesey13\Projects\Rust\RustyHello\hello.exe 
[New Thread 2540.0x2118]
warning: FTH: (2540): *** Fault tolerant heap shim applied to current process. This is usually due to previous crashes. ***

[New Thread 2540.0xf64]
[New Thread 2540.0x1a94]
[New Thread 2540.0x23a8]
thread '<main>' panicked at 'called `Option::unwrap()` on a `None` value', ../src/libcore\option.rs:366

Breakpoint 1, 0x000000000043e04a in rust_panic ()
(gdb) bt
#0  0x000000000043e04a in rust_panic ()
#1  0x0000000000405508 in sys_common::unwind::begin_unwind_inner::hd8664d70d0e8892a0ks ()
#2  0x0000000000405c6c in sys_common::unwind::begin_unwind_fmt::h70b08f84bc0aa0aa6js ()
#3  0x000000000043e1a3 in rust_begin_unwind ()
#4  0x00000000004634b9 in panicking::panic_fmt::hd4a3f538cd5921dcZEK ()
#5  0x000000000045d911 in panicking::panic::h11474eb550eeee87wDK ()
#6  0x0000000000401567 in option::_$LT$impl$GT$::unwrap::unwrap::h7269811594158191321 ()
#7  0x000000000040150d in main::h584a48e1b57381e6eaa ()
#8  0x0000000000447fc9 in sys_common::unwind::try::try_fn::h13142058720409824620 ()
#9  0x0000000000431582 in sys_common::unwind::try::inner_try::hd8672c6ed8d8693fyhs ()
#10 0x0000000000447ed8 in rt::lang_start::h2fd7bb830c345727Hix ()
#11 0x00000000004015bb in main ()
