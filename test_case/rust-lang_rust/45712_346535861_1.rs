 rust
pub struct MyStruct<'a, T: 'a>(&'a T);


pub trait MyTrait {
    fn my_method<'a, T, I>(&mut self, into: I)
        where I: Into<MyStruct<'a, T>>;
}
