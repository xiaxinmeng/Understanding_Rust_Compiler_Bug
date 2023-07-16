
macro_rules! test(
    ($arg:ident) => (foo::$arg);
)

fn main() {
    test!(bar);
}
