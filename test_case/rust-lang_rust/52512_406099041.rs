rust
#![feature(decl_macro)]
#![allow(unused)]

mod m {
    fn f() {
        macro_rules! mac { () => {
            mod std {
                pub macro panic() {}
            }
        }}
        
        std::panic!(); // should be an ERROR: ambiguous prelude vs macro-expanded module
                            // not an error now due to prelude being ignored
        
        mac!();
    }
}

fn main() {}
