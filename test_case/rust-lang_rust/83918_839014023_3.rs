rust
// The pattern becomes refutable because this is now matching on an integer inside the slice pattern.
if let [a, b, c @ 5..=9, ..] = xs {
    println!("captured {}, {}, and {}!", a, b, c); // prints: captured 13, 1, and 5!
} else {
    println!("whoops");
}
