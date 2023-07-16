rs
#[repr(transparent)]
pub struct Array([u64; 2]);
#[no_mangle]
pub unsafe extern "C" fn function(arg: Array) -> Array {
    arg
}
