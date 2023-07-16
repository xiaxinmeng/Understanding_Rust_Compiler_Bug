 rust
pub trait MyImpl<A> {
    fn myimpl_fun<B>(&self, _: A) -> Option<B> {
        None
    }   
}

pub struct MyStruct<B> {
    _data: B,
}

impl<B> MyImpl<B> for MyStruct<B> {}
