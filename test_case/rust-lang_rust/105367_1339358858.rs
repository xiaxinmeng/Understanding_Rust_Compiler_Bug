rust
fn foo() {
    let mut x = "qwerty";  // x is declared as `mut`

    str::make_ascii_lowercase(&mut x); // bad error message
    x.make_ascii_lowercase(); // good error message
}
