 rust
fn c<T: Copy>(t: T) -> (T, T) { (t.clone(), t) }

fn f() {}

fn main() {
    c(0);
    c(f);
}
