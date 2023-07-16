rust
pub trait Outlives<'this> {}

impl<'this, T> Outlives<'this> for T where T: 'this {}

fn assert_static_via_hrtb<G>(_: G) where for<'a> G: Outlives<'a> {}

fn main() {
    let local = 0;
    assert_static_via_hrtb(&local);
}
