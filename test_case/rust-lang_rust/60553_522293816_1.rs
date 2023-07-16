rust
#[repr(C)]
struct foo_t {
    pub discr: u8,
    pub payload: foo_u,
}

union foo_u {
    b: u16,
    c: c_t,
}

#[repr(C)]
struct c_t {
    a: u32,
    b: u16,
}
