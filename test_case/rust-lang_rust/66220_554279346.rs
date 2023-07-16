
#[repr(transparant)]
struct Opaque<T>(T);

extern "C" { fn c_func(ptr: *mut c_void) -> size_t};

pub fn wrapper<T>(ptr: &mut Opaque<T>) -> usize {
unsafe {c_func(ptr as *mut _ as *mut c_void)} as usize
}
