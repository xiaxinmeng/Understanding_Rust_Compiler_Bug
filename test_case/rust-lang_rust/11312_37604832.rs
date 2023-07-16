
#[no_mangle] 
#[no_split_stack]
pub extern "C" fn __morestack() { rust_stack_exhausted() }
