rust
// Should Print:
// local!! 1
// local!! 2
//
// Does Print:
// local!! 1
//

#![allow(dead_code)]

#[repr(C, u8)] enum Space { Local(f32), Screen }

#[repr(C)]
struct SpaceRepr {
    tag: Tag,
    padding: [u8; 3],
    payload: Payloads,
}

#[repr(C)]
union Payloads {
    local: f32,
}

#[repr(u8)]
enum Tag {
    Local,
    Screen,
}

fn handle_space(space: &Space) {
    if let Space::Local(val) = *space {
        println!("local!! {}", val);
    }
}

fn main() {
    // We should be allowed to store any garbage in padding, right?
    let local1 = SpaceRepr { tag: Tag::Local, padding: [0, 0, 0], payload: Payloads { local: 1.0 }};
    let local2 = SpaceRepr { tag: Tag::Local, padding: [1, 2, 3], payload: Payloads { local: 2.0 }};
    
    let local1_ref: &Space = unsafe {::std::mem::transmute(&local1)};
    let local2_ref: &Space = unsafe {::std::mem::transmute(&local2)};
    
    handle_space(local1_ref);
    handle_space(local2_ref);
}
