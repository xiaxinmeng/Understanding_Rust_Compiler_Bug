rust
use std::arch::x86::__cpuid;

pub unsafe fn foo() -> u32 {
    // get manufacture ID
    let res = __cpuid(0);
    res.ebx
}
