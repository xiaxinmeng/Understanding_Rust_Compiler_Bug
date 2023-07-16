 rust
trait Num {}

impl Num for i32 {}
impl Num for f32 {}

fn vec2s<T: Num>(a: T) {}

fn main() {
    // Removing either of these lines, or explicitly typing one of the literals stops the ICE from happening
    vec2s(0.0);
    vec2s(0);
}
