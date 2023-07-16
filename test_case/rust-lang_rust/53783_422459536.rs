rust
let x = x as *mut u8;
let y = y as *mut u8;
for i in 0..mem::size_of::<T>() {
    let tmp = *x;
    *x = *y;
    *y = tmp;
}
