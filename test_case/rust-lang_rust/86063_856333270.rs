
#0  panic_semihosting::panic (info=0x20001be0) at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-semihosting-0.5.6/src/lib.rs:79
#1  0x08000ff4 in core::panicking::panic_fmt (fmt=...)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:92
#2  0x08000fd6 in core::panicking::panic (expr=...)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:50
#3  0x08003286 in core::ops::bit::{{impl}}::shl (self=<optimized out>, other=<optimized out>)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/bit.rs:459
#4  0x080036d2 in compiler_builtins::int::shift::Ashl::ashl<u64> (self=<optimized out>, shl=4294967264)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.45/src/int/shift.rs:9
#5  0x08004280 in compiler_builtins::int::shift::__ashldi3 (a=2305873796076149728, b=134240100)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.45/src/int/shift.rs:80
#6  0x0800428a in compiler_builtins::int::shift::__aeabi_llsl::__aeabi_llsl (a=2305873796076149728, b=134240100)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.45/src/macros.rs:242
#7  0x0800338e in core::ops::bit::{{impl}}::shl (self=536878048, other=32)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/bit.rs:459
#8  0x080035b8 in compiler_builtins::int::mul::Mul::mul<i128> (self=0, rhs=<optimized out>)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.45/src/int/mul.rs:22
#9  0x080041fe in compiler_builtins::int::mul::__multi3 (a=10635389834159551675033706660840348640, b=<optimized out>)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.45/src/int/mul.rs:108
#10 0x0800421c in compiler_builtins::int::mul::__multi3::__multi3 (a=10635389834159551675033706660840348640, b=<optimized out>)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.45/src/macros.rs:286
#11 0x08001d82 in core::fmt::num::udiv_1e19 (n=<optimized out>)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/num.rs:663
#12 0x08001b78 in core::fmt::num::fmt_u128 (n=2305873795539271681, is_nonnegative=<optimized out>, f=0x20001ea0)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/num.rs:599
#13 0x08001b58 in core::fmt::num::{{impl}}::fmt (self=<optimized out>, f=0x1)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/num.rs:586
#14 0x08000156 in core::fmt::num::{{impl}}::fmt (self=0x20001f98, f=0x20001ea0)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/num.rs:191
#15 0x08000106 in core::fmt::{{impl}}::fmt<i128> (self=<optimized out>, f=0x20001c00)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:2031
#16 0x08002140 in core::fmt::run (fmt=0x20001ea0, arg=0x80045ac, args=...)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:1155
#17 0x08002004 in core::fmt::write (output=..., args=...)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:1123
#18 0x08000308 in core::fmt::Write::write_fmt<cortex_m_semihosting::hio::HStderr> (self=0x20000004 <cortex_m_semihosting::export::HSTDERR+4>, args=...)
    at /home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:184
#19 0x08000684 in cortex_m_semihosting::export::hstderr_fmt::{{closure}} ()
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-semihosting-0.3.7/src/export.rs:49
#20 0x0800060a in cortex_m::interrupt::free<closure-0,core::result::Result<(), ()>> (f=...)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-0.7.2/src/interrupt.rs:64
#21 0x0800063e in cortex_m_semihosting::export::hstderr_fmt (args=...)
    at /home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-semihosting-0.3.7/src/export.rs:44
#22 0x08000220 in bug_example::__cortex_m_rt_main () at src/main.rs:11
#23 0x080001b4 in bug_example::__cortex_m_rt_main_trampoline () at src/main.rs:9
