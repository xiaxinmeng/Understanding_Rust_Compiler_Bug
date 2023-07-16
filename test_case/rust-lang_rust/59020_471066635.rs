rust
use std::thread;
use std::time::Duration;
use std::time;

fn main() {
    println!("{:?}", time::Instant::now());

    let t1 = thread::spawn(|| {
        println!("{:?}", time::Instant::now());
        let sleep = Duration::new(0,100_000);
        for _ in 0..100 {
            println!("Parking1");
            thread::park_timeout(sleep);
        }
    });

    let t2 = thread::spawn(|| {
        println!("{:?}", time::Instant::now());
        let sleep = Duration::new(0,100_000);
        for _ in 0..100 {
            println!("Parking2");
            thread::park_timeout(sleep);
        }
    });

    t1.join().expect("Couldn't join thread 1");
    t2.join().expect("Couldn't join thread 2");
}
