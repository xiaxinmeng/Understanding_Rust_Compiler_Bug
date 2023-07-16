rust
#[no_mangle]
pub fn got() {
    let x: u64 = 0x0123456789ABCDEF;
    show(&x); let x = x;
    show(&x); let x = x;
    show(&x);
}

#[no_mangle]
pub fn expect() {
    let x: u64 = 0x0123456789ABCDEF;
    show(&x);
    show(&x);
    show(&x);
}

#[no_mangle]
#[inline(never)]
fn show(x: &u64) { println!("(0x{:x})0x{:x}", x as *const _ as usize, x); }
