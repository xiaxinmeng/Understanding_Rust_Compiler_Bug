rust
pub type Oink = (u128, ());

pub fn oink(a: Oink, b: Oink) -> Oink {
    (a.0 + b.0, ())
}
