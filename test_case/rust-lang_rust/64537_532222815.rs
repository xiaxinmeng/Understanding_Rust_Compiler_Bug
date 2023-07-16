rust
#![feature(const_generics)]

struct S;

trait T {
    fn x<const I: usize>();
    fn y<const I: usize>();
}

impl T for S {
    fn x<const I: usize>() {}
    fn y<const I: usize>() {
        <S as T>::x::<{I}>()
    }
}
