rust
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[repr(C, packed)]
#[derive(Copy, Clone, Serialize, Deserialize)]
struct Header {
    len: u16,
    kind: u8
}

fn main() {
}

