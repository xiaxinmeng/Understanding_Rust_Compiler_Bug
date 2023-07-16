
fn foo<const SIZE: usize>(_: impl FnMut()) { }
fn main() {
    foo::<1usize>(||{});
}
