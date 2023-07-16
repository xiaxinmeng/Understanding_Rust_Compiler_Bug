
trait A { }
trait B: A { }

impl<T: A> T: B { }
