
macro_rules! foo (() => (
    2 => "two"
));

fn main() {
    match 2 {
        foo!(),
        _ => "not two",
    }
}
