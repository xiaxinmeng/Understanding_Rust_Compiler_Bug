
#[repr(packed)]
pub struct ObjInd {
    _header: u32,
    field: u32,
}

pub unsafe fn test(obj: *const ObjInd) {
    assert_eq!((*obj).field, 123); // "reference to packed field is unaligned"
}
