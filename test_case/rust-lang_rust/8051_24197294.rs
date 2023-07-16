 rust
use std::rt::rtio::RtioTimer;
use std::rt::io::Timer;

fn main() {
    let maybe_timer = Timer::new();
    let mut t = maybe_timer.expect("Expected a timer");
    t.sleep(1000);
}
