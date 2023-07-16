rust
use std::fmt::Debug;

fn is_debug<T: Debug>(_: T) { }

fn main() {
    let mut v: Option<_> = None;
    
    let generator = async move {
        is_debug(v);
    };
    
    if false {
        // Commenting out this line will yield an error, because the type
        // of `v` is not specified.
        v = Some(String::new());
    }
    
    drop(generator);
}
