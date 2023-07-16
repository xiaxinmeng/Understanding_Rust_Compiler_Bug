rust
mod first {
    mod a {
        pub(crate) struct Foo(
            // Change `pub(super) bool` to `pub(crate) bool`
            // to cause a breaking change.
            //
            // Just swap which line is commented out and which is included:
            pub(super) bool  // this is fine
            // pub(crate) bool  // this is a breaking change
        );
    }
    
    mod b {
        pub(crate) struct Foo{}
    }
    
    pub(crate) use a::*;
    pub(crate) use b::*;  
}

mod second {
    pub struct Foo();
}

use first::*;  // *** NOT A `pub use` ***
pub use second::*;
