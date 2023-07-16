rust
mod helpers {
    pub trait IntExt {
        fn id(self) -> Self;
    }
    
    impl IntExt for i32 {
        fn id(self) -> Self { self }
    }
}

use helpers::IntExt;

pub mod user {
    use super::*;
    
    pub fn foo(x: i32) -> i32 {
        x.id()
    }
}
