
pub struct Box<T: ?Sized, A = Global>(Unique<T>, PhantomData<A>);
