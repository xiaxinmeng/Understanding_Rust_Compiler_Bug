 rust
use std::thread;
use std::old_io::timer::sleep;
use std::time::Duration;
fn main() {
    for i in 1..5000 {
        thread::spawn(move || {
            println!("{}", i);
            sleep(Duration::seconds(10));
        });
    }
}
