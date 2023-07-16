rust
use std::io::Read;

struct Foo {
    inner: Option<Box<dyn Read + 'static>>,
}

impl Foo {
    fn member<'a>(&'a mut self) {
        let x: Option<&'a mut (dyn Read + 'static)> = self.inner.as_deref_mut();
        bar(x);
    }
}

fn bar<'b>(read: Option<&'b mut (dyn Read + 'b)>) {
    
}


fn main() {
  let mut foo = Foo { inner: None };
  foo.member();
}
