rust
// Before
sink! {
    "string"any_suffix // OK
    10u123 // OK
    10.0f123 // OK
    0b10f32 // OK
    999340282366920938463463374607431768211455999 // OK
}

#[cfg(FALSE)]
fn configured_out() {
    "string"any_suffix // ERROR
    10u123 // ERROR
    10.0f123 // ERROR
    0b10f32 // ERROR
    999340282366920938463463374607431768211455999 // ERROR
}

fn main() {
    "string"any_suffix // ERROR
    10u123 // ERROR
    10.0f123 // ERROR
    0b10f32 // ERROR
    999340282366920938463463374607431768211455999 // ERROR
}
