rust
pub unsafe fn test(obj: *const ObjInd) {
    println!("{}", { (*obj).field });
}
