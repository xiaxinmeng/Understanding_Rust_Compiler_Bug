rust
pub trait My {
    fn f(&self) -> i32;
}

#[repr(transparent)]
pub struct Wrapper<T: ?Sized>(T);

pub struct My1<T: My + ?Sized> {
    field: i8,
    my: Wrapper<T>,
}

type DynMy1 = My1<dyn My>;

pub fn run(d: &DynMy1) -> &Wrapper<dyn My> {
    &d.my
}
