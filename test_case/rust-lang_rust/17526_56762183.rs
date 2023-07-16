 rust
fn test(iter: &mut ObjIter) {
    for mut obj_ref in iter {
        let obj = obj_ref.get_obj();
    }
}
