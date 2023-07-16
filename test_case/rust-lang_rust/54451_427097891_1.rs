 rust
#[no_mangle] // only used to cause a compile error
static FOO: () = ();

// Proxy for memory mapped peripheral register block
pub struct Zst { _0: () }

impl std::ops::Deref for Zst { // omitted: DerefMut impl
    type Target = u32; // simplified; this should be a struct
    fn deref(&self) -> &u32 {
        const ADDRESS: usize = 0x4000_0000; // peripherals are located at known addresses
        unsafe { &*(ADDRESS as *const u32) }
    }
}

pub fn peripheral_once() -> Option<Zst> {
    // omitted the logic that makes this produce a `Some` value the first time
    // it's called and `None` in subsequent calls
    Some(Zst { _0: () })
}
