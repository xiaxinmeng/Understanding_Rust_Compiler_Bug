rust

#[repr(align(64))]
struct A;

struct P<'a, T: ?Sized + 'a> {
    p: &'a T,
    a: A,
}

fn main() {
    use std::mem::size_of;
    dbg!(size_of::<(P<'static, u8>, *const (/*vtable*/))>());
    dbg!(size_of::<P<'static, dyn Sync>>());
}
