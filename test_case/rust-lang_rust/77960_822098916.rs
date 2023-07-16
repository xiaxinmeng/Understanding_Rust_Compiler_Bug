rust
fn bar<S: Into<String>>(s: S) {
    some_work(); // s: S
    more_work(); // s: S
    let s: String = s.into(); // s: String
    even_more_work(&s); // s: String
    yet_even_more_work(s); // s: String
}
