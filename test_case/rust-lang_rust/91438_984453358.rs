rust
pub trait My {
    fn f(&self) -> i32;
}

pub struct My1<T: My + ?Sized> {
    field: i8,
    my: T,
}

type DynMy1 = My1<dyn My>;

pub fn run(d: &DynMy1) -> &dyn My {
    &d.my
}
