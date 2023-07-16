rs
fn main() {
    let a: *const u8;
    let b: *const u8;
    {
        let v: [u8; 16] = [core::hint::black_box(0); 16];
        a = &(v[0]);
    }
    {
        let v: [u8; 16] = [core::hint::black_box(0); 16];
        b = &(v[0]);
    }
    println!("{a:?} == {b:?} evaluates to {}", a==b);
    println!("{a:?} == {b:?} evaluates to {}", a==b);
}
