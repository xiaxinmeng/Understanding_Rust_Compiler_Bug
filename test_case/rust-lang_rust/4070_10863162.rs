
trait A { fn f() }
trait B: A { fn g() }

impl S: B {
    fn f() {}
    fn g() {}
}

fn foo<T: B>(bar: T) { bar.f(); }
