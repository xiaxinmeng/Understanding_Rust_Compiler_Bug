rust
pub struct E;

#[allow(unreachable_code)]
pub fn foo(never: !) {
    <E as From<_>>::from(never);
}
