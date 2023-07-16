rust
trait Id {
    type This: ?Sized;
}

impl<T: ?Sized> Id for T {
    type This = T;
}

fn assert_same<A, B>() where A: Id<This = B> {}
