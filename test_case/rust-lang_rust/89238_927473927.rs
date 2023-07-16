rust
fn main() {
    // A sequence of unique floats, printed by Rust with varying numbers of significant digits.
    let a: Vec<f32> = vec![
        0.1000734,
        0.100073405,
        0.10007341,
        0.10007342,
        0.10007343,
        0.100073434,
        0.10007344,
        0.10007345,
        0.10007346,
        0.100073464,
        0.10007347,
        0.10007348,
        0.10007349,
        0.100073494,
    ];

    for f in a {
        println!("{:.1$}", f, f32::DIGITS as usize); // All chopped down to the same string: 0.100073
    }
}
