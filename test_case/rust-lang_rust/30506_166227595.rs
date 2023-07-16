
macro_rules! foo{
    ($x: expr) => {$x}
}

fn main() {
    foo!(bar());
}
