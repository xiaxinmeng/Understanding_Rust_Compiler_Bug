rust
trait ChainExt: Error {
    fn chain(&self) -> Chain
    where
        Self: Sized + 'static,
    {
        <dyn Error + 'static>::chain(self)
    }
}

impl<E: Error> ChainExt for E {}

fn _assert_object_safe(_: &dyn ChainExt) {
    
}

fn walk_source_chain(error: &(impl Error + 'static)) {
    for e in error.chain() {
        println!("{}", e);
    }
}
