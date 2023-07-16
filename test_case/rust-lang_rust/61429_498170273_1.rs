rust
#![feature(asm)]

#[derive(Default)]
struct ThreadContext;


fn gt_switch(new: *const ThreadContext) {
       
        unsafe {
        asm!("
        mov     0x00($0), %rsp
        ret
       "
        : 
        : "m"(new)
        );

    }
}

fn main() {
    let ctx = ThreadContext::default();

    gt_switch(&ctx);
}
