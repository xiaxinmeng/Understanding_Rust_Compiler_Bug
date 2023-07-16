rust
#![feature(const_type_name)]
#![feature(type_alias_impl_trait)]

use core::any::type_name;

fn read_integer_or_whatever() -> i32 { todo!() }

const fn bla<T>() -> usize { type_name::<T>().len() }

fn z(_: &'static [usize]) {}

fn f() {
    let x = read_integer_or_whatever();
    
    // Here's where the magic happens:
    type TypeOfX = impl Sized;
    if false {
        let _def_use = move || -> TypeOfX { x };
        loop {}
    }

    // Note that `z(&[bla::<i32>()])` also doesn't work because
    // calls to const fns are not static promoted
    const B: usize = bla::<TypeOfX>();
    z(&[B]);
}
