rust
fn main() {
    let offset = test();
    println!("0x{offset:X}");
}

fn test() -> usize {
    let mut v = Vec::<u8>::with_capacity(usize::MAX);
    let slice = v.spare_capacity_mut();
    let front = &slice[0] as *const _ as usize;
    let back = &slice[usize::MAX - 1] as *const _ as usize;
    return back - front;
}
