rust
#[repr(interoperable)]
struct F64Inside {
    a: u8,
    b: f64, // forced to have size == align == 8
    c: [f64; 3], // forced to have size == 24, align == 8
}
