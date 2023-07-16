rust
struct Floaty {}

impl From<f64> for Floaty {
    fn from(value: f64) -> Floaty {
        value.into()
    }
}

fn main() {
    let _: Floaty = 1.0.into();
}
