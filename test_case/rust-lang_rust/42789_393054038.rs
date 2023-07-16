rust
mod unique_token {
    use std::sync::atomic::{AtomicBool, Ordering};

    // With a private field so it can only be constructed inside this module
    pub struct Token(());
    
    // Stores whether we have already created a token
    static GUARD : AtomicBool = AtomicBool::new(false);
    
    impl Token {
        pub fn new() -> Option<Token> {
            if GUARD.compare_and_swap(false, true, Ordering::AcqRel) == false {
                Some(Token(()))
            } else {
                None
            }
        }
        
        pub fn take_two(a: &Token, b: &Token) {
            // There's only ever one token so these have to be the same
            assert_eq!(a as *const _, b as *const _,
                "How can there be two tokes?!?");
        }
    }
}
