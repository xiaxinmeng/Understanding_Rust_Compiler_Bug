rust
pub struct My1<T: My + ?Sized> {
    field: i8, // <= default 1 alignment
    my: Wrapper<T>, // <= could dynamicly change alignment of My1 ?
}
