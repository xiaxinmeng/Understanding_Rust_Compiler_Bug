rust
#[test]
#[should_panic(expected = "Invalid color: Color { red: NaN, green: 0, blue: 0, alpha: 0 }. See the color module documentation for more information.")]
fn rejects_invalid_background_color() {
    // ...
}
