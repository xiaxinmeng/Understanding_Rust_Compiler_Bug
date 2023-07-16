 rust
use std::rc::Rc;

#[derive(Clone)]
enum CachedMir<'mir> {
    Ref(&'mir String),
    Owned(Rc<String>),
}

impl<'mir> CachedMir<'mir> {
    fn get_ref<'a>(&'a self) -> &'a String {
        match *self {
            CachedMir::Ref(r) => r,
            CachedMir::Owned(ref rc) => &rc,
        }
    }
}

fn main() { }
