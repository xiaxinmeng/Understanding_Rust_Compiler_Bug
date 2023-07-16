
macro_rules! foo{
    ($x: expr) => {$x.bar()}
}

fn main() {
    foo!(42);
}
