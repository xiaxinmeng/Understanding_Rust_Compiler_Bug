rust
fn main() {
    let a_closure = |num| {
        return num+1.0
    };
    println!("{}", a_closure(1.0f64)); // no longer inferring float type
}
