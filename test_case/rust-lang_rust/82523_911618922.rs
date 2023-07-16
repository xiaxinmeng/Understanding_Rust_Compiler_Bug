rust
pub unsafe fn test(concat: *const Obj) {
    println!("{}", (&*addr_of!((*concat).field)).len());
}
