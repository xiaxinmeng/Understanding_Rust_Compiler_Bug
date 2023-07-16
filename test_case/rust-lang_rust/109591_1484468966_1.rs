rust
use std::any::Any;

fn into_any<T: 'static>(t: T) -> impl Any {
    t
} 

fn main() {
    let _ = [into_any(0u8), into_any(0u16)];
}
