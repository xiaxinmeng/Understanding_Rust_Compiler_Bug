rust
#[repr(u8)]
pub enum Tag {
    Undefined = 0,
    Null = 1,
    I32 = 2,
    F64 = 3,
    Str = 4,
    False = 5,
    True = 6,
    Array = 7,
    Object = 8,
    Reference = 9,
    Function = 10,
    FunctionMut = 12,
    FunctionOnce = 13,
    UnsafeTypedArray = 14,
    Symbol = 15
}

#[repr(C)]
pub struct SerializedValue< 'a > {
    data_1: u64,
    data_2: u32,
    tag: Tag,
    phantom: PhantomData< &'a () >
}
