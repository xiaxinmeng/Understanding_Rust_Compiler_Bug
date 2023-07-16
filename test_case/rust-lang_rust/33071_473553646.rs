rust
fn main() {
    let mut var = 0i32;
    let borrow = &mut var;
    || {
        let borrow_borrow = &borrow;
        *borrow = 1;
        let _use = borrow_borrow;
    };
}
