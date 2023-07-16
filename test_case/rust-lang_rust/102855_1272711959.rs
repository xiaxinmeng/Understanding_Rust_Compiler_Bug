rust
use self::animation::File;

mod animation {
    mod plugins {
        use std::fs::*;
        
        #[derive(Default)]
        pub struct File;
    }

    pub use self::plugins::*;
}
