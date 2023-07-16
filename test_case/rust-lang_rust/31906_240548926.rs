
pub const CONST_C_STR1: &'static [u8] = b"test1\0";

fn compare_extern(value: * const c_char) {
    let from_client = unsafe {CStr::from_ptr(value)};
    match from_client.to_bytes() {
        CONST_C_STR1 => println!("test1"),
    }
}

fn main() {
    let test = CString::new("test2").unwrap();
    compare_extern(test.as_ptr());
}
