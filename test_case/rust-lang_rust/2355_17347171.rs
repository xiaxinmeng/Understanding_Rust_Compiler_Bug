
trait foo<A> { }

fn bar<A : foo<A> >(_x: A) { /* ... */ }
