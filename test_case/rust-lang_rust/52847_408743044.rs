rust
use std::thread;
use std::time::Duration;

fn main() {
    let threads: Vec<_> = (0..10).map(|_| {
        thread::Builder::new()
            .stack_size(100 * 1024 * 1024)
            .spawn(|| {
                thread::sleep(Duration::from_secs(3600));
            }).unwrap()
    }).collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
