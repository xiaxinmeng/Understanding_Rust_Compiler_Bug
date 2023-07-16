rust
// This causeds an ICE when targeting i686-apple-darwin (from a
// x86_64-apple-darwin host).

#![crate_name = "cocoa"]
#![crate_type = "rlib"]

#![allow(non_upper_case_globals)]

pub mod appkit {
    use std::fmt::{self};

    #[repr(u64)]
    pub enum NSEventType {
        NSEventTypePressure = 34,
    }

    pub struct NSEventMask {
        bits: u64,
    }

    pub const NSEventMaskPressure: NSEventMask =
        NSEventMask{bits: 1 << (NSEventType::NSEventTypePressure as u64),};

    impl fmt::Debug for NSEventMask {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if NSEventMaskPressure.bits != 0 {
                match f.write_str("NSEventMaskPressure") {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(From::from(err))
                    }
                }
            }
            Ok(())
        }
    }
}
