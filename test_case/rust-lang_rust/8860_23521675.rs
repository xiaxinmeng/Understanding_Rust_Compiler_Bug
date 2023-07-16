 rust
struct S { i: int }
impl Drop for S {
    fn drop(&self) {
        println("drop");
    }
}

fn f(ref _s: S) {}

fn main() {
    let s = S { i: 42 };
    f(s);
}
