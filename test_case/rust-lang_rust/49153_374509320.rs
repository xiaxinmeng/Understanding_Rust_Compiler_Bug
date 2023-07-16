rust
extern "C" {
    pub static __ImageBase: u8;
}

pub static FOO: &'static u8 = unsafe { &__ImageBase };
