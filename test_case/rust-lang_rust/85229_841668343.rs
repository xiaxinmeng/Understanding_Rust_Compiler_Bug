rust
#![feature(const_generics, const_evaluatable_checked)]

pub fn print_const<const N: usize>() {
    println!("{}", N);
}

struct Const<T>(T);
impl<T> Const<T> {
    const N: usize = 5;
}

pub fn print_thing<T>() {
    print_const::<{Const::<T>::N}>()
}
