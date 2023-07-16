rust
fn do_shift(val: &u64, amt: &u8) -> u64 {
    *val << *amt
}

fn shift_test() {
    let my_u64: u64 = 1;
    let my_shift: u8 = 0;

    if black_box(do_shift(black_box(&my_u64), black_box(&my_shift))) != 1 {
        panic!()
    }
}
