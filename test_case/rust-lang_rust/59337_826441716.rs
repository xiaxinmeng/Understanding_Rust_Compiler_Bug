rust
fn test1<T>(_: &T) { }
fn test2<T>(_: T) { }

fn main() {
    let _: for<'a> fn(&'a u32) = test1;
    let _: for<'a> fn(&'a u32) = test2; // error: one type is more general than the other
}
