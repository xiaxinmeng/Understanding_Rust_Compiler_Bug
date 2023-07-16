 rust
extern {
    fn ffi_fn();
}

struct FFI {
    f: Option<unsafe extern fn()>,
}

fn main() {
    let _ffi1 = FFI {
        f: Some(ffi_fn as unsafe extern "C" fn()),
    };
}
