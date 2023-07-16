rust
struct Foo<T>(T);

trait GoodBye {
    type Forget;
}
impl<T> GoodBye for T {
    type Forget = ();
}

trait NeedsWf<'a, 'b> {
    type Assoc;
}

impl<'a, 'b> NeedsWf<'a, 'b> for Foo<<&'a &'b () as GoodBye>::Forget> {
    type Assoc = &'a &'b ();
}

fn main() {}
