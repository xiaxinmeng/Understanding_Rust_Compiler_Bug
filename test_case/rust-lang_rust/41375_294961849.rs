rust
#![crate_type="staticlib"]
#[repr(C)] pub struct Big { data: [u8; 64] }
#[repr(C)] pub struct Small { data: u8 }

#[no_mangle]
pub extern "C" fn test(_: Big, _: Big, _: Big, _: Big, _: Big, _: Big, s: Small) {
    println!("{}", s.data);
}
