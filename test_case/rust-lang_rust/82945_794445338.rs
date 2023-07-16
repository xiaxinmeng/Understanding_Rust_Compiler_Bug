
^C
Program received signal SIGINT, Interrupt.
cortex_m_rt::HardFault_ (ef=0x2000fd30)
    at /home/birk/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:563
563	        atomic::compiler_fence(Ordering::SeqCst);
(gdb) where
#0  cortex_m_rt::HardFault_ (ef=0x2000fd30)
    at /home/birk/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:563
#1  <signal handler called>
#2  0x0c004c96 in core::intrinsics::copy_nonoverlapping<u8> ()
    at /rustc/cb75ad5db02783e8b0222fee363c5f63f7e2cf5b//library/core/src/intrinsics.rs:1866
