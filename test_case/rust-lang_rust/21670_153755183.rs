
use std::thread;
// use std::any::TypeId; // uncomment this line to make the compile work

fn main() {
    let child = thread::spawn(|| {
    });
    child.join().unwrap_err().get_type_id();
}
