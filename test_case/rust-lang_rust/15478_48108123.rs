 rust
extern crate native;
extern crate rustrt;

use std::os;
use std::str;

static PAD: [u8, ..0x180] = [0, ..0x180];
static DATA: [u8, ..0x1000] = [b' ', ..0x1000];

fn main() {
    let addr = DATA.as_ptr() as uint;
    println!("{:x}: {}", addr, str::from_utf8(DATA).unwrap());
    // Make sure DATA is aligned to the start of a page (adjust PAD to ensure this).
    assert!((addr & 0xfff) == 0);

    // When this (immediately) goes out of scope (immediately), it will munmap DATA's page.
    os::MemoryMap {
        data: addr as *mut u8,
        len: 0x1000,
        kind: os::MapVirtual
    };

    // Open a file an map it in the same space pointed to by DATA.
    let f = native::io::file::open(&"/etc/passwd".to_c_str(), rustrt::rtio::Open, rustrt::rtio::Read).ok().unwrap();
    let _mm = os::MemoryMap::new(0x1000, [
        os::MapFd(f.fd()),
        os::MapReadable,
        os::MapAddr(DATA.as_ptr())
    ]).unwrap();

    // TAADAA!
    println!("{}", str::from_utf8(DATA).unwrap());

    // Keep the padding alive.
    println!("{}", PAD.as_slice());
}
