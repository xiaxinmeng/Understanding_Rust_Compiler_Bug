 rust
macro_rules! make_vec(
    (a $e1:expr $($(, a $e2:expr)*)*) => ([$e1 $($(, $e2)*)*]);
);

fn main() {
    let _ = make_vec!(a 1, a 2, a 3);
}
