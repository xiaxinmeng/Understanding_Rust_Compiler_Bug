
use std::borrow::{Borrow, Cow};

#[derive(Clone)]
struct Unit;

fn main() {
    let unit_ref = &Unit;
    let unit_cow: Cow<'_, Unit> = Cow::Borrowed(unit_ref);
    // This does not work (we should use `.borrow()` here):
    // let _: &Unit = Unit.as_ref();
    // But this works (but should not be guaranteed to work):
    let _: &Unit = unit_cow.as_ref();
    // Semantically correct is to use `.borrow()` instead of `.as_ref()` in both cases:
    let _: &Unit = Unit.borrow();
    let _: &Unit = unit_cow.borrow();
}
