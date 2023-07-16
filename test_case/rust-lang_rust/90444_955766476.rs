rs
pub struct B(());

enum Thing {}

impl From<fn((), &[B], ())> for Thing {
    fn from(_: fn((), &[B], ()) -> Self {}
}
