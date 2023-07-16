rust
#![allow(unused_variables)]
extern crate num_cpus;

fn main() {
    let cpus = num_cpus::get();
    let mut threads = Vec::with_capacity(cpus);

    for i in 0..cpus {
        threads.push(::std::thread::spawn(|| {
            loop {
                let number = 20000_f64;
                let using = number.sqrt();
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
