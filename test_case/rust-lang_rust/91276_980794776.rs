rust
(gdb) c
Continuing.
^C
Program received signal SIGINT, Interrupt.
lib::__wfi () at asm/lib.rs:50
50	asm/lib.rs: No such file or directory.
(gdb) bt
#0  lib::__wfi () at asm/lib.rs:50
#1  0x000043b6 in cortex_m::asm::wfi () at /Users/tdh/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.3/src/asm.rs:55
#2  atsamd_hal::sleeping_delay::{impl#1}::delay_us<atsamd_hal::rtc::Rtc<atsamd_hal::rtc::Count32Mode>, u32> (self=0x2002f59c, us=<optimized out>)
    at /Users/tdh/.cargo/git/checkouts/atsamd-d272bcb1b045fea7/635c583/hal/src/sleeping_delay.rs:55
#3  0x000042d4 in delay_toggle::profile_target<atsamd_hal::sleeping_delay::SleepingDelay<atsamd_hal::rtc::Rtc<atsamd_hal::rtc::Count32Mode>>> (delay=0x4000240a)
    at embedded-profiling-examples/src/bin/delay_toggle.rs:72
#4  0x0000470a in delay_toggle::__cortex_m_rt_main () at embedded-profiling-examples/src/bin/delay_toggle.rs:62
#5  0x000045fa in delay_toggle::__cortex_m_rt_main_trampoline () at embedded-profiling-examples/src/bin/delay_toggle.rs:22
(gdb) frame 1
#1  0x000043b6 in cortex_m::asm::wfi () at /Users/tdh/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.3/src/asm.rs:55
55	    call_asm!(__wfi())
(gdb) frame 0
#0  lib::__wfi () at asm/lib.rs:50
50	asm/lib.rs: No such file or directory.
