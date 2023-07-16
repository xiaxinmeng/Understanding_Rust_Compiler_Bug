rust
#![feature(thread_local)]

#[thread_local]
static mut FOO: u32 = 3;

fn foo() -> &'static u32 {
    unsafe { &*(&A as *const _) }
}

pub mod a {
    pub fn bar() -> u32 {
        *super::foo()
    }
}
