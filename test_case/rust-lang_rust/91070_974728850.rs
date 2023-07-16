rust
pub mod before {
    extern "C" {
        pub static GLOBAL1: [u8; 1];
    }

    pub unsafe fn do_something_with_array() -> u8 {
        GLOBAL1[0]
    }
}

pub mod inner {
    extern "C" {
        pub static GLOBAL1: u8;
    }

    pub unsafe fn call() -> u8 {
        GLOBAL1 + 42
    }
}
