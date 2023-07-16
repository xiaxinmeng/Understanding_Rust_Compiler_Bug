rust
#[repr(align(8))] struct Align8;

fn main() {
    let _: Box<Align8> = unsafe { Box::from_raw(3 as *mut _) };
    let _: Box<Align8> = unsafe { Box::from_raw(0 as *mut _) };
}
