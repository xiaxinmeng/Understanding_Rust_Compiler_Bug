rust
#![feature(const_fn)]
#![feature(const_type_id)]

use std::any::TypeId;
use std::any::type_name;
use std::mem::transmute;

const fn same_type<A: 'static, B: 'static>() -> bool {
    unsafe { transmute::<_, u64>(TypeId::of::<A>()) == transmute::<_, u64>(TypeId::of::<B>()) }
}

fn print_if_equal<T: 'static, U: 'static>() {
    if same_type::<T, U>() {
        println!("{} == {}", type_name::<T>(), type_name::<U>());
    } else {
        println!("{} != {}", type_name::<T>(), type_name::<U>());
    }
}

fn main() {
    print_if_equal::<usize, u32>();
    print_if_equal::<usize, usize>();
}
