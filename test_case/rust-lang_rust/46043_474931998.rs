rust
#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Header {
    request: u32,
    flags: u32,
    size: u32,
}
