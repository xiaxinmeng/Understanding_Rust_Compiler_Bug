rust
trait Sink<Item> {
    type Error;
}


trait UsingSink {
    type Item;

    fn use_sink<S>(ch: &mut S) -> Result<Self::Item, S::Error>
    where
        S: Sink<Self::Item>,
        // Commenting out the following line in the declaration and the impl
        // makes the code compile
        S::Error: Send;
        
}

struct Foo;

// --------- The Problem is in this impl ---------

impl UsingSink for Foo {
    type Item = ();

    fn use_sink<S>(ch: &mut S) -> Result<Self::Item, S::Error>
    where
        S: Sink<Self::Item>,
        // commenting out this line makes the code compile
        S::Error: Send,
    {
        todo!()
    }
}
