Rust
#[derive(Clone)]
struct BoxFoo;
impl std::ops::Deref for BoxFoo {
    type Target = Foo;
    fn deref(&self) -> &Foo { panic!() }
}

impl std::ops::DerefMut for BoxFoo {
    fn deref_mut(&mut self) -> &mut Foo { panic!() }
}


#[derive(Clone)]
struct Foo(Option<BoxFoo>, Option<BoxFoo>);

fn test(f: &mut Foo) {
  match *f {
    Foo(Some(ref mut left), Some(ref mut right)) => match **left {
      Foo(Some(ref mut left), Some(ref mut right)) => panic!(),
      _ => panic!(),
    },
    _ => panic!(),
  }
}

fn main() {
}
