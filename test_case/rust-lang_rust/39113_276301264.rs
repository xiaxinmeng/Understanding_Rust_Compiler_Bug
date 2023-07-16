
mod detail {
    pub struct UnnameableStruct;
    pub struct UnnameableTrait;
}

pub fn f<T: UnnameableTrait>() -> UnnameableStruct {}
