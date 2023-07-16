rust
#[repr(C, packed)]
pub struct Thing<const N: usize> {
    bytes: [u8; N],
    len: usize
}

fn main() {
    let thing = Thing { bytes: *b"foo", len: 3 };
    let bytes_ptr = std::ptr::addr_of!(thing.bytes) as *const u8;
    let s = unsafe { 
        std::str::from_utf8_unchecked(
            std::slice::from_raw_parts(bytes_ptr, thing.len)
        ) 
    };
    println!("s={}", s);  // it prints 'foo'
}
