 rust
extern crate libc;

pub mod raw {
    pub use self::ffi::tcflag_t;

    #[allow(dead_code, non_camel_case_types)]
    mod ffi {
        pub type tcflag_t = ::libc::c_uint;

        pub const CSTOPB_: ::libc::c_uint = 64;
        pub const CREAD_: ::libc::c_uint = 128;
    }

    pub const CSTOPB: tcflag_t = self::ffi::CSTOPB_ as tcflag_t;
    pub const CREAD: tcflag_t = self::ffi::CREAD_ as tcflag_t;
}

pub mod control {
    use raw;

    #[cfg(error)]
    #[repr(u32)]
    #[deriving(Show)]
    pub enum Bit {
        CSTOPB = raw::CSTOPB,
        CREAD = raw::CREAD,
    }

    #[cfg(not(error))]
    #[repr(u32)]
    pub enum Bit {
        CSTOPB = raw::CSTOPB,
        CREAD = raw::CREAD,
    }
}

fn main() {}
