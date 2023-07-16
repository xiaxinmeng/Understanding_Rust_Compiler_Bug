rust
// Commented code does not compile
fn main() {
    let v = 1;
    let x: *const u32 = &v;
    let y: &u32 = &1;
    let _ = dbg!(x == y);
    // let _ = dbg!(PartialEq::eq(x, y));
    // let _ = dbg!(PartialEq::eq(x, &y));
    // let _ = dbg!(PartialEq::eq(&x, y));
    // let _ = dbg!(PartialEq::eq(&x, &y));
    let _ = dbg!(PartialEq::eq(&x, &(y as _)));

    // But that can't be a desugaring...
    // let _ = dbg!(v == y);
    let _ = dbg!(&v == y);
    // let _ = dbg!(PartialEq::eq(&v, &(y as _)));
}
