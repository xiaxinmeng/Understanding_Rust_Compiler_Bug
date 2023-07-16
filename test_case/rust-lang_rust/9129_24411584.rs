
fn main() {
    macro_rules! breakme(
        ($value: expr) => {
            break 'foo;
        }
    )

    breakme!("foo");
}
