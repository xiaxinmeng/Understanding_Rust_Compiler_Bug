rust
struct E;
impl From<!> for E {
    fn from(x: !) -> E { x }
}
fn foo(never: !) {
    <E as From<_>>::from(never);
}
