rust
#[repr(packed)]
pub struct Obj {
    pub field: String,
}

pub unsafe fn test(concat: *const Obj) {
    println!("{}", (*concat).field.len());
    //             ^^^^^^^^^^^^^^^ "reference to packed field is unaligned"
}
