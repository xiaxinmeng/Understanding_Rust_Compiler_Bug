rust
#![feature(const_type_id)]
#![feature(core_intrinsics)]

pub struct GetTypeId<T>(T);

impl<T: 'static> GetTypeId<T> {
    pub const VALUE: u64 = std::intrinsics::type_id::<T>();
}

const fn check_type_id<T: 'static>() -> bool {
    matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)
}

fn main() {
    assert!(check_type_id::<usize>());
}
