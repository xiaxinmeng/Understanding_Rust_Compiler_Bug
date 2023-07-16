rust
use std::time::Duration;
use std::cmp::PartialEq;
use fun_library::some_duration;

fn main() {
    let d = some_duration();
    if d.eq(&Duration::ZERO) {
        println!("what could be less than an instant?");
    }
}
