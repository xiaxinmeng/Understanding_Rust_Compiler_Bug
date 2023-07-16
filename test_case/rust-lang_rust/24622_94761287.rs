 Rust
pub trait Foo<'a> {
    type Bar;
}

impl<'a, T> Foo<'a> for T { type Bar = &'a T; }

fn denormalise<'a, T>(t: &'a T) -> <T as Foo<'a>>::Bar {
    t
}

pub fn free_and_use<T: for<'a> Foo<'a>,
                    F: for<'a> FnOnce(<T as Foo<'a>>::Bar)>(x: T, f: F) {
    let y;
    'body: loop { // lifetime annotations added for clarity
        's: loop { y = denormalise(&x); break }
        drop(x);
        return f(y);
    }
}

pub fn main() {
    free_and_use("foo".to_string(), |s| println!("{}", s));
}
