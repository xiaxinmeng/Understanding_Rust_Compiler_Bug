rust
#[no_mangle]
pub use my_helper_lib::exported_fn; // not exported

#[no_mangle]
extern "C" fn another_fn() {} // this is exported fine

fn main() {
   let lib = libloading::Library::new("my_lib.so").unwrap();
}
