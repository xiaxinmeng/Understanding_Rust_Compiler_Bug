rust
static X: u8 = 123;
fn main() {
    let x = vec![None::<&[u8]>];
    let r = transmute::<&Option<&[u8]>, &Option<&[u8]>>(&x[0]);
    drop(x);
    let _x = vec![&X as *const _ as usize, 1000000];
    println!("{:?}", r.unwrap());
}
