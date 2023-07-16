rust
fn t(mut ts: Pin<&mut TestStruct>) {
    let d = &mut *ts;
    let x = &mut d.a;
    let y = &mut d.b;

    *y = *x;
}
