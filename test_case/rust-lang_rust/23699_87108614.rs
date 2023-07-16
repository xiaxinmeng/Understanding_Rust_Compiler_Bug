 rust
fn test<T>(t: T) { }

fn main() {
    let t = test as fn (i32);
    t(0i32);
}
