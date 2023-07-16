rust
macro_rules! check {
    (1. $($a:lifetime,)* $b:block) => {};
    (2. $($b:block,)* $a:lifetime) => {};
}
fn main() {
    check!(1. 'a, 'b, {});
    check!(2. {}, {}, 'a);
}
