
fn add_callback(cb: &FnOnce()){
    unsafe {
        let closure: raw::TraitObject = mem::transmute(cb);
    }
}
