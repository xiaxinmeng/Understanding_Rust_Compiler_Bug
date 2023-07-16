rust
async fn test<'a>() { // `'a`, being a lifetime parameter, must be longer than the entire function body
    let f = |_: &'a str| {}; // the argument is required to outlive `'a`
    f(&String::new()); // an argument of shorter lifetime is passed!
}
