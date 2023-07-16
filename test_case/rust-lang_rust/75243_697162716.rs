rust
trait ArrayExt {
    fn map<F: FnMut(i32) -> i32>(&self, f: F);
}

impl ArrayExt for [i32; 1] {
    fn map<F: FnMut(i32) -> i32>(&self, f: F) {}
}

fn main() {
    [4].map(|x| x);
}
