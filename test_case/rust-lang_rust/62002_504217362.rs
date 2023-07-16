rust
mod some_private_mod {
    #[no_mangle]
    pub extern "C" fn __panic_internal() {
        println!("the one from core");
    }
}

extern "C" {
    fn __panic_internal();
}

pub fn function_in_core() {
    unsafe { __panic_internal() }
}
