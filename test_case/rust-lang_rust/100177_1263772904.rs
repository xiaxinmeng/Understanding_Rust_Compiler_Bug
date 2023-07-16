rust
trait Sink<Item> {
    type Error;

    fn foo(&mut self) -> Result<Item, Self::Error> {
        todo!()
    }
}

trait Channel<Item>: Sink<Item> {}

// This works
fn use_channel<C>(ch: &mut C) -> Result<(), C::Error>
where
    C: Channel<()>,
    <C as Sink<()>>::Error: Send,
{
    // ch.foo()
    todo!()
}

trait UsingChannel {
    type Item;

    fn use_channel<C>(ch: &mut C) -> Result<Self::Item, C::Error>
    where
        // Commenting out the following line in the declaration and the impl
        // makes the code compile
        <C as Sink<Self::Item>>::Error: Send,
        C: Channel<Self::Item>;
}

struct Foo;

// --------- The Problem is in this impl ---------

impl UsingChannel for Foo {
    type Item = ();

    fn use_channel<C>(ch: &mut C) -> Result<Self::Item, C::Error>
    where
        C: Channel<Self::Item>,
        // commenting out this line makes the code compile
        <C as Sink<Self::Item>>::Error: Send,
    {
        // ch.foo()
        todo!()
    }
}
