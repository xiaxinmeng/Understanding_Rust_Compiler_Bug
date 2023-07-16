 rust
#![feature(thread_local)]

extern {
    #[thread_local]
    #[link_name = "errno"]
    pub static errno: i32;
}

pub fn foo() -> i32 {
    unsafe { errno }
}
