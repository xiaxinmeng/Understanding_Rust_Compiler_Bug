rust
pub trait Wrapable {
    fn dyn_wrap(self) -> Box<dyn Wrapable>;
}
pub struct DynWrapper(pub Box<dyn Wrapable>);

pub struct Wrapper<T>(T);

impl<T: 'static> Wrapable for Wrapper<T> {
    fn dyn_wrap(self) -> Box<dyn Wrapable> {
        Box::new(Wrapper(self))
    }
}

fn main() {
    DynWrapper(Box::new(Wrapper(())));
}
