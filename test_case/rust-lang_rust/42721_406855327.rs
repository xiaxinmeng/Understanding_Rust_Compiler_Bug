
enum MyOption<T> {
  None,
  Some(T)
}

impl<T> Debug for MyOption<T> where T: ?Debug {
          match self {
            None => write!(f, "Unset"),
            Some(_) => write!(f, "An unprintable value")         
        }
}
 
#[derive(Debug)]
struct Foo {
  a: u32,
  b: MyOption<SomethingWithoutDebug>
}
