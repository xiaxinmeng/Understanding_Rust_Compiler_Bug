 rust
#[feature(macro_rules)];

enum T {
    A(int),
    B(uint)
}

macro_rules! test(
    ($e:expr) => (
        fn foo(t: T) -> int {
            match t {
                A(y) => $e,
                B(y) => $e
            }
        }
    )
)

test!(10 + (y as int))

fn main() {
    foo(A(20));
}
