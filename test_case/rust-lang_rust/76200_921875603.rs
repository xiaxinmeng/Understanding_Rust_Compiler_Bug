
#![feature(generic_const_exprs)]

fn foo<T: Default + Copy, const L: usize>(_: [T; L]) -> [T; L + 1] {
    [T::default(); L + 1]
}

fn bar(x: [u8; 2]) -> [u8; 3] {
    foo::<u8, 2>(x)
}

fn baz<T: Default + Copy>(x: [T; 2]) -> [T; 3] {
    foo::<T, 2>(x)
}

fn main() {}
