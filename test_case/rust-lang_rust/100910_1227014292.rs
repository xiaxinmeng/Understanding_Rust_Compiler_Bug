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

impl<'a, 'b> NeedsWf<'a, 'b> for Foo<<&'a &'b () as GoodBye>::Forget>
where
    &'a &'b (): GoodBye, // this line prevents us from normalizing in the param env.
{
    type Assoc = &'a &'b ();
}

fn needs_wf<'a, 'b, T: NeedsWf<'a, 'b>>() {}

fn foo<'a: 'a, 'b: 'b>(_: &'b String) {
    needs_wf::<'a, 'b, Foo<()>>();
}

fn main() {
    let x = String::from("hello");
    foo::<'static, '_>(&x);
}
