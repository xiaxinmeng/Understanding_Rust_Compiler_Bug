 rust
pub fn all<T>(predicate: &fn(T) -> bool, iter: &fn(f: &fn(T) -> bool)) -> bool {
    for iter |x| {
        if !predicate(x) {
            return false
        }
    }
    true
}

fn main() {
    all(|x: uint| x < 6, |f| std::uint::range(1, 6, f));
}
