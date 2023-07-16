rust
use std::thread;
// use std::any::TypeId; // <- actually compiles as-is

fn main() {
    let child = thread::spawn(|| {
    });
    child.join().unwrap_err().get_type_id();
}
