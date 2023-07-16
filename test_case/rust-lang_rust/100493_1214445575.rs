rust
use std::ffi::CStr;
use std::os::raw::c_char;

fn main() {
    fn read<'a>(ptr: *const c_char, bogus: &'a ()) -> String {
        unsafe { CStr::from_ptr::<'a>(ptr) }.to_str().unwrap().to_owned()
    }
    fn write<'a>(ptr: *mut c_char, bogus: &'a mut ()) {
        // whatever
    }
    fn foo<'a>(ptr: *mut c_char, bogus: &'a mut ()) {
        let first = read(ptr, &*bogus);
        write(ptr, bogus);
        let second = read(ptr, &*bogus);
    }
}
