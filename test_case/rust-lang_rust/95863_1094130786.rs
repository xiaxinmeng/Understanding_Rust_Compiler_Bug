rust
// For indirection
pub trait With {
    type F;
}

impl With for i32 {
    type F = fn(&str);
}

pub struct V<T: With>(<T as With>::F);

fn v<T: With>(f: <T as With>::F) -> V<T> {
    V(f)
}

fn main() {
    // A function item to coerce
    fn f(_: &str) {}

    let _: V<i32> = v(f);
}
