rust
#[inline(never)]
pub fn calc(v: P<X>) -> P<X> {
    let a = P::<X>::new(v.get_value() * 1.234_f32);
    let b = P::<X>::new(5.0_f32);

    a / b
}
