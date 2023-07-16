rust
#![feature(const_type_id)]

use std::any::{type_name, TypeId};

pub struct GetTypeId<T>(T);

impl<T: 'static> GetTypeId<T> {
    pub const VALUE: TypeId = TypeId::of::<T>();
}

#[macro_export]
macro_rules! typeid {
    ($t:ty) => {
        $crate::GetTypeId::<$t>::VALUE
    };
}

const fn same_type<T: 'static, U: 'static>() -> bool {
    match typeid!(T) {
        typeid!(U) => true,
        _ => false,
    }
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
