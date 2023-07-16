
use std::boxed::Box;

#[derive(Debug)]
pub struct Foo<'a>(pub &'a str);

pub trait IntoBox {
    fn into_box<'a>(self) -> Box<Fn(Foo) -> String>;
}

impl<B> IntoBox for B where B: Fn(Foo) -> String + 'static {
    fn into_box(self) -> Box<Fn(Foo) -> String> { Box::new(self) }
}

fn direct_into_box<B: Fn(Foo) -> String + 'static>(b: B) -> Box<Fn(Foo) -> String> {
    Box::new(b)
}

fn main() {
    // Doesn't work
    let x = IntoBox::into_box(|i| format!("{:?}", i) );

    // Works
    let y = IntoBox::into_box(|i: Foo| format!("{:?}", i) );

    // Also works
    let z = direct_into_box(|i| format!("{:?}", i) );
}
