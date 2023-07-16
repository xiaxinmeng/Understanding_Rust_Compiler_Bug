rust
#[path = "foo"]
mod blah {
    #![path] // `rustc` errors even though it doesn't matter (the outer one "wins")
}
