
#[repr(usize)]
enum MyWeirdOption<T> {
    None,
    Some = std::mem::size_of::<T>(),
    //~^ ERROR constant expression depends on a generic parameter
}
