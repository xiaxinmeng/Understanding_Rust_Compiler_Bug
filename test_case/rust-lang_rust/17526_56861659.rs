 Rust
fn test(iter: &mut ObjIter) {
    for obj_ref in iter {
        let mut obj_ref = obj_ref; // Note the explicit reborrow
        obj_ref.get_obj();
    }
}
