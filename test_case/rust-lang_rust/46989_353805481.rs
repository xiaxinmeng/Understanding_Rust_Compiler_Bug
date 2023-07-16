rust
fn assert_partialeq<T: PartialEq<T>>() {}

fn main() {
    assert_partialeq::<fn(&i32)>();
}
