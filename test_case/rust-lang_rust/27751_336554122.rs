rust
fn data_pointer(object: &SomeTrait) -> *const () {
    let ptr: *const SomeTrait = object;
    ptr as *const ()
}
