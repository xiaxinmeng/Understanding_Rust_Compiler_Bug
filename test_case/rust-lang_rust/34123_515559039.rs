rust
#![feature(intrinsics)]

fn main(){
    let transmute = std::intrinsics::transmute;
    let assign: unsafe extern "rust-intrinsic" fn(*const i32) -> *mut i32 = transmute;
}
