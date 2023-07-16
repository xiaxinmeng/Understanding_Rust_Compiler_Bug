rust
pub struct S1 { _x: f64, _y: f64 }

pub fn indirect(get_pair: fn () -> S1) {
    let _s1 = get_pair();
}
