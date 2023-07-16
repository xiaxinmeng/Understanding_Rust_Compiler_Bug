diff
pub struct RectangleShape<'a>(&'a ());

pub struct Human<'a> {
    image: RectangleShape<'a>,
}

pub trait Entity {
    fn image(&mut self) -> &mut RectangleShape;
}

impl<'a> Entity for Human<'a> {
    fn image(&mut self) -> &'static mut RectangleShape { 
+       assert_static(&self); // <~ :(
        &mut self.image
    }
}

fn assert_static<T: 'static>(_: &T) {}
