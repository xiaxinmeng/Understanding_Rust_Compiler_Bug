rust
#![feature(asm_sym)]
fn func<T>() {}

fn blarg<T>() {
    unsafe {
        std::arch::asm!("/* {} */", sym func::<T>);
    }
}

fn foo() {
    blarg::<i32>();
}
