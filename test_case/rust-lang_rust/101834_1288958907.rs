rust
trait Indirect: FnMut(i32) -> String {}

impl<T: FnMut(i32) -> String> Indirect for T {}

fn takes_closure<T: Indirect>(mut f: T, i: i32) -> String {
    f(i)
}

fn main() {
    println!("{}", takes_closure(|i| i.to_string(), 3));
}
