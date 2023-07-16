rust
pub fn fn0() -> bool {
    let mut pair = (1, false);
    let ptr = core::ptr::addr_of_mut!(pair.1);
    let mut ret = pair.1 <= unsafe { *ptr };
    pair = (1, false);
    unsafe {
        *ptr = ret | ret;
    }
    ret = !pair.1;
    return ret;
}

pub fn main() {
    println!("{}", fn0());
}
