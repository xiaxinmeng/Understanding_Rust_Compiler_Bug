 rust
#![feature(unboxed_closures)]
#![feature(core)]

use std::ops::Mul;

struct Fun<F: Fn(T) -> T, T>(F, std::marker::PhantomData<T>);

impl<F, T> FnOnce<(T,)> for Fun<F, T> where F: Fn(T) -> T {
    type Output = T;

    extern "rust-call" fn call_once(self, args: (T,)) -> T {
        self.call(args)
    }
}

impl<F, T> FnMut<(T,)> for Fun<F, T> where F: Fn(T) -> T {
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> T {
        self.call(args)
    }
}

impl<F, T> Fn<(T,)> for Fun<F, T> where F: Fn(T) -> T {
    extern "rust-call" fn call(&self, (t,): (T,)) -> T {
        (self.0)(t)
    }
}

impl<T, F1, F2> Mul<Fun<F2, T>> for Fun<F1, T>
where 
    F1: Fn(T) -> T, 
    F2: Fn(T) -> T {

    type Output = Compose<Fun<F1, T>, Fun<F2, T>, T>;
    fn mul(self, rhs: Fun<F2, T>) -> Compose<Fun<F1, T>, Fun<F2, T>, T> {
        Compose{f1: self, f2: rhs, _ph: std::marker::PhantomData}
    }
}


struct Compose<F1, F2, T> where F1: Fn(T) -> T, F2: Fn(T) -> T {
    f1: F1,
    f2: F2,
    _ph: std::marker::PhantomData<T>,
}


impl<T, F1, F2> FnOnce<(T,)> for Compose<F1, F2, T> where F1: Fn(T) -> T, F2: Fn(T) -> T {
    type Output = T;

    extern "rust-call" fn call_once(self, args: (T,)) -> T {
        self.call(args)
    }
}

impl<T, F1, F2> FnMut<(T,)> for Compose<F1, F2, T> where F1: Fn(T) -> T, F2: Fn(T) -> T {

    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> T {
        self.call(args)
    }
}

impl<T, F1, F2> Fn<(T,)> for Compose<F1, F2, T> where F1: Fn(T) -> T, F2: Fn(T) -> T {

    extern "rust-call" fn call(&self, (t,): (T,)) -> T {
        (self.f1)((self.f2)(t))
    }
}

fn main() {
    let f1 = Fun(|i: isize| i * 2, std::marker::PhantomData::<isize>);
    let f2 = Fun(|i: isize| i - 1, std::marker::PhantomData::<isize>);
    let f3 = f1 * f2;
    println!("{}", f3(3));
}
