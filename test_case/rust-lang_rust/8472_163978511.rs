 rust
#![feature(stmt_expr_attributes)]

macro_rules! foo(
        () => {
            #[allow(unused_unsafe)]
            unsafe { use std::mem; let () = mem::transmute(()); }
        }   
);

fn main() {
    unsafe { foo!(); }
    foo!();
}
