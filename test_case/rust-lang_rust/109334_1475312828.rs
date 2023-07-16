rust
pub mod with_trans {
    #[repr(transparent)]
    pub struct Handle(());

    extern "C" {
        /// Advances the enumeration list
        pub fn foreign(a: Handle);
    }
}

pub mod without_trans {
    pub struct Handle(());

    extern "C" {
        pub fn foreign(a: Handle);
    }
}

fn main() {}
