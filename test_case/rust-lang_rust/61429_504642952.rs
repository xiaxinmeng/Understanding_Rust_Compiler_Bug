rust
#![feature(asm)]

struct ThreadContext {
    rsp: u64,
    r15: u64,
}

fn gt_switch(new: *const ThreadContext) -> ! {
    unsafe {
        asm!("mov     $0, %r15" : : "*m"(&(*new).r15) : "r15");
        asm!("mov     $0, %rsp" : : "*m"(&(*new).rsp) : "rsp");
        asm!("ret");
        std::hint::unreachable_unchecked()
    }
}

fn main() {
    let ctx = ThreadContext{rsp: 0x80, r15: 0x88};

    gt_switch(&ctx);
}
