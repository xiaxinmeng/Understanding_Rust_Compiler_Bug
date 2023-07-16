
fn size_of<T>() -> T where T: Sized {
    let v : [u8; ::std::mem::size_of::<T>()] = unimplemented!();
    v
}
