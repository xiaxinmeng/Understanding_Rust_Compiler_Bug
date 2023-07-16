
pub trait FromSlice<'a> {
    fn new(struct_reader : &'a [u8]) -> Self;
}
