rust
pub struct Foo<'a>(&'a usize);

impl<'a> ::std::cmp::PartialEq for Foo<'a> {
    fn eq(&self, _: &Foo<'a>) -> bool { true  }
    fn ne(&self, _: &Foo<'a>) -> bool { false  }
}

fn should_equal<'a, 'b>(a: Foo<'a>, b: &'b usize) {
    assert!(a == Foo(b));
}
