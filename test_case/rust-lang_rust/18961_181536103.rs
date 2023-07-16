 rust
//struct S1<T> { s: Option<T> }
struct S1<T> { s: *mut T }

struct Foo<'a> {
    bar: S1<&'a str>
}

impl<'a> Foo<'a> {
    pub fn new() -> Foo<'a> {  // '
        //Foo { bar: S1 { s: None } }
        Foo { bar: S1 { s: std::ptr::null_mut() } }
    }

    pub fn baz(&'a self) -> Option<isize> {
        None
    }

    pub fn qux(&'a mut self, retry: bool) {
        let opt = self.baz();
        if retry { self.qux(false); }
    }
}

pub fn main() {
   let mut foo = Foo::new();
   foo.qux(true);
}
