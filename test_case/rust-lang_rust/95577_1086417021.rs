rust
fn main() {
    // the user should know better than to not provide an argument.
    // it's clearly documented right here in the source, after all.
    let x = std::env::args()
        .skip(1)
        .next()
        .unwrap();

    println!("hello {x}!")
}
