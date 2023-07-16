rust
fn main() {
    let a_closure = |num| {
        return num+"bar" // no number literal type inference
    };
    println!("{}", a_closure("foo".to_string()));
}
