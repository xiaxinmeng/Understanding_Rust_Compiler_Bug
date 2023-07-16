 rust
// 16 bytes
struct RawData {
    pub data: [u8, ..16]
}

// 16 bytes
struct StructuredData {
    pub data: u16,
    pub _align: i64,
}

fn addr_to_sockaddr() -> StructuredData {
    let mut storage = StructuredData { data: 0, _align: 0 };
    unsafe {
        // Try to set the fields by directly setting memory
        let raw = &mut storage as *mut _ as *mut RawData;
        for i in range(0, 16) {
            (*raw).data[i] = i as u8;
        }
    }
    // Then return it as a struct which is largely padding
    return storage;
}

fn main() {
    // Get it back
    let addr = addr_to_sockaddr();
    // Convert to raw data
    unsafe {
        let addrp = &addr as *const _ as *const RawData;
        // The padding is now zeroes!
        println!("raw data {}", (*addrp).data.as_slice());
    }
}
