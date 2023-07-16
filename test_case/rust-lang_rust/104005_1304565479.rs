rust
// ... same as above
fn test<'a>() {
    let _: fn(&'a str) -> _ = extend_lt;
}
