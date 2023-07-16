rust
// run-pass

// check that we handle subtyping between types with a different binding structure correctly,
// especially in LLVM - see issues #55976 & #47638

pub struct Foo<T> {
    t: T,
    force_in_memory_layout: [u8; 64]
}

pub trait MirrorT<'a> {
    type Image;
}

impl<'a, T: Copy> MirrorT<'a> for T {
    type Image = T;
}

pub trait H<T> {}
impl<T> H<T> for () {}

// This is the "core problem": this function performs subtyping from
// `&Foo<Box<for<'b> Fn(&'b u8) -> u32>>` to `&Foo<Box<Fn(&'a u8) -> u32>>`
pub fn abc<'a, T: for<'s> MirrorT<'s> + 'static> (
    x: &Foo<Box<dyn for<'b> H<<T as MirrorT<'b>>::Image>>>) 
    -> &Foo<Box<dyn H<<T as MirrorT<'static>>::Image>>> 
{ 
    x
}

// check that it runs
fn main() {
    match abc::<u8>(&Foo {
        t: Box::new(()),
        force_in_memory_layout: [1; 64]
    }) {
        _v => {
//            assert_eq!((v.t)(0u8), 42);
//            assert_eq!(&v.force_in_memory_layout as &[u8], &[1u8; 64] as &[u8]);
        }
    };
}
