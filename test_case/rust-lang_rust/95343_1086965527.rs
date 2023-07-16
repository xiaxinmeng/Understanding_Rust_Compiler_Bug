rust
// main.rs

macro_rules! decl {
    (#[doc = $tt:tt]) => {
        println!("decl macro: #[doc = {}]", stringify!($tt));
    };
}

fn main() {
    decl! {
        ///"
    }
    proc! {
        ///"
    }
}
