rust
type CSSFloat = f32;

pub enum ViewportPercentageLength {
    Vw(CSSFloat),
    Vh(CSSFloat),
    Vmin(CSSFloat),
    Vmax(CSSFloat),
}

impl ViewportPercentageLength {
    fn try_sum(&self, other: &Self) -> Result<Self, ()> {
        use self::ViewportPercentageLength::*;
        Ok(match (self, other) {
            (&Vw(one), &Vw(other)) => Vw(one + other),
            (&Vh(one), &Vh(other)) => Vh(one + other),
            (&Vmin(one), &Vmin(other)) => Vmin(one + other),
            (&Vmax(one), &Vmax(other)) => Vmax(one + other),
            _ => return Err(()),
        })
    }
}

#[no_mangle]
pub extern "C" fn sum_them(
    one: &ViewportPercentageLength,
    other: &ViewportPercentageLength,
    out: &mut ViewportPercentageLength,
) -> bool {
    match one.try_sum(other) {
        Ok(v) => {
            *out = v;
            true
        }
        Err(()) => false,
    }
}

fn main() {}

