rust
extern crate objrs;

use objrs::objrs;

#[objrs(class, root_class)]
pub struct NSArray<T: objrs::marker::Class + ?Sized>;

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl<T: objrs::marker::Class + ?Sized> NSArray<T> {
    #[objrs(selector = "firstObject")]
    pub fn first_object<'a>(&'a self) -> Option<&'a T> {}
}
