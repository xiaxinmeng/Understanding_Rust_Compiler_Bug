rust
trait Foo {
    fn make_it() -> Self where Self: Sized;
    fn len(&self) -> usize;
    fn write(&mut self, s: &str);
}

impl Foo for String {
    fn make_it() -> Self {
        String::new()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn write(&mut self, s: &str) {
        self.extend(s.chars());
    }
}


fn main() {
    let bar = "123";
    let val: String = {
        #[inline]
        fn some<F: Foo, C: FnOnce(&mut F)>(f: C) -> F {
            let mut tmp: F = Foo::make_it();
            f(&mut tmp);
            tmp
        }

        some(|tmp| {
            Foo::len(tmp);
            Foo::write(tmp, bar);
        })
    };
}

