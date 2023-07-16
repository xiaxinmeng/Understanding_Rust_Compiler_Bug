rust
fn main() {
    [1, 2, 3].sort_by(|tuple| panic!());
    [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
}
