 rust
use core::Check;

fn main() {
    let x = Value;
    x.check();
}

struct Value;

impl Check for Value {
    fn check(&self) -> bool {
        true
    }
}

pub mod core {
    pub use self::private::Check;
    mod private {
        pub trait Check {
            fn check(&self) -> bool;
        }
    }
}
