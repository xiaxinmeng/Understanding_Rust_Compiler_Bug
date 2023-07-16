rust
pub struct Action {}

impl<F> From<F> for Action
where
    for<'a> F: Fn(&'a ()),
{
    fn from(_: F) -> Self {
        Self {}
    }
}

fn main() {
    Action::from(|_| {});
    //Action::from(|_: &()| {});
}
