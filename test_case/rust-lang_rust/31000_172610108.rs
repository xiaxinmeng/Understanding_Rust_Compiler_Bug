 rust
// compile-flags: -C no-prepopulate-passes

// Make sure that simple cases of dst.clone_from_slice(src) produce memcpy
// (relies on loop idiom recognize).

#![crate_type = "lib"]

// CHECK-LABEL: @copy_slice
#[no_mangle]
pub fn copy_slice(src: &[u8], dst: &mut [u8]) {
    // CHECK: memcpy
    dst.clone_from_slice(src);
}
