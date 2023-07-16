rust
// library is same as the original issue

// in a new crate:
fn main() -> Result<(), impl core::fmt::Debug> {
    use rustice::sub;

    let x = sub::make_struct()?;

    assert!(matches!(x, sub::En::A(sub::B { inner: vec![], .. })));
    Ok::<(), &'static str>(())
}
