
struct Foo<'a>
{
    a: [u8; std::mem::size_of::<&mut u8>()]
}
