rust
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let count = 33_000;

    // "min" defaults to 100 so we get the real lowest timing, instead of 0.
    let mut min = Duration::from_secs(100);
    let mut max = Duration::from_secs(0);
    let mut total = Duration::from_secs(0);
    let mut handles = Vec::with_capacity(count);

    for i in 0..count {
        let start = Instant::now();
        let handle = thread::Builder::new()
            .stack_size(1 * 1024 * 1024)
            .spawn(move || assert!(i <= count)) // Just some dummy work
            .unwrap();

        let duration = start.elapsed();

        handles.push(handle);

        if duration < min {
            min = duration;
        } else if duration > max {
            max = duration;
        }

        total += duration;
    }

    let avg = total / (count as u32);

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "min: {:2} µsec, avg: {:2} µsec, max: {:2} µsec",
        min.as_micros(),
        avg.as_micros(),
        max.as_micros()
    );
}
