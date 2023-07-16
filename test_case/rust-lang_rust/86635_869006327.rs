rs
trait R { type Assoc; }

// Does not need `for` for the error
trait Foo<'a> where &'a Self: R<Assoc = Self>, Self: 'a {}
trait Foo2 where for<'a> &'a Self: R<Assoc = Self> {}

fn do_thing<'a, T: Foo<'a>, T2: Foo2>(_: (T, T2)) {
    
}
