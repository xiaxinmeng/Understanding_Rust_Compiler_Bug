rust
mod outer {
    mod inner {
        pub(crate) fn hi() {
            println!("HI");
        }
    }
    pub use inner::*;
}

use outer::*;
fn main() {
    hi();
}
