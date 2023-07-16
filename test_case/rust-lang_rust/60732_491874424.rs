rust
#![feature(arbitrary_enum_discriminant,const_raw_ptr_deref)]

#[allow(dead_code)]
#[repr(u8)]
enum Enum {
    Unit = 3,
    Tuple(u16) = 2,
    Struct {
        a: u8,
        b: u16,
    } = 1,
}

impl Enum {
    const unsafe fn tag(&self) -> u8 {
        *(self as *const Self as *const u8)
    }
}

const UNIT_TAG:   u8 = unsafe { Enum::Unit.tag() };
const TUPLE_TAG:  u8 = unsafe { Enum::Tuple(5).tag() };
const STRUCT_TAG: u8 = unsafe { Enum::Struct{a: 7, b: 11}.tag() };

fn main() {
    assert_eq!(3, UNIT_TAG);
    assert_eq!(2, TUPLE_TAG);
    assert_eq!(1, STRUCT_TAG);
}
