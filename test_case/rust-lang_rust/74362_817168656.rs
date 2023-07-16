rust
#![feature(const_type_id)]

#[inline(always)]
pub const fn is_bool<T: 'static>()->bool{
    use std::any::TypeId;

    const BOOL_ID: TypeId = TypeId::of::<bool>();
    match TypeId::of::<T>(){
        BOOL_ID => true,
        _ => false,
    }
}

pub fn good()->bool{
    is_bool::<bool>()
}

pub fn bad()->bool{
    is_bool::<u8>()
}
