Rust
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

struct Foo {
    child: Option<Box<Foo>>,
}

fn rnd() -> bool {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen()
}

impl Foo {
    fn foo1<'s>(&'s mut self) -> Option<&'s mut Foo> {
        match self.child {
            None => None,
            Some(ref mut chld) => Some(self),
        }
    }

    fn foo2<'s>(&'s mut self) -> Option<&'s mut Foo> {
        match self.child {
            None => None,
            Some(ref mut chld) => Some(if rnd() { &mut *chld } else { self }),
        }
    }

    fn foo3<'s>(&'s mut self) -> Option<&'s mut Foo> {
        match self.child {
            None => None,
            Some(ref mut chld1) if !rnd() => Some(self),
            Some(ref mut chld2) => Some(&mut *chld2),
        }
    }

    fn foo4<'s>(&'s mut self) -> Option<&'s mut Foo> {
        match self.child {
            None => None,
            Some(ref mut chld1) if rnd() => Some(&mut *chld1),
            Some(ref mut chld2) => Some(self),
        }
    }
}

fn main() {}
