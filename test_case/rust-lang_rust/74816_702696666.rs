Rust
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

trait UnsafeCopy {
    type Copy<T>: Copy + Drop + ?Sized + std::fmt::Debug = [i32];
    
    fn copy<T: ?Sized>(x: &Self::Copy<T>) {
        let blah: Self::Copy<T> = *x;

        let b = Box::new(blah);
        dbg!(b);
    }
}

impl <T> UnsafeCopy for T {}

fn main() {
    let b = Box::new([1]) as Box<[i32]>;
    let copy = <Box<[i32]>>::copy::<[i32]>(&b);
}
