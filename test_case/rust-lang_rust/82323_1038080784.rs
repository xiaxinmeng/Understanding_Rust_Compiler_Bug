rs
struct Struct<T>(T);
impl Struct<T>
where
    T: Copy,
{
    fn method(v: Vec<u8>) { v.len(); }
}
