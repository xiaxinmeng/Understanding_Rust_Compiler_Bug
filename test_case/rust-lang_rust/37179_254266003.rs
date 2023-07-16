
use std::thread;

fn compute_primes() -> i64 {
        let mut a: i64 = 2;
        let mut count: i64 = 0;
        while count < 200000 {
            let mut b: i64 = 2;
            let mut prime = true;
            while (b * b) <= a {
                if a % b == 0 { prime = false; break; }
                b += 1;
            }
            if prime { count += 1; }
            a += 1;
        }
        count
}

fn main() {
    let child1 = thread::spawn(|| {
        compute_primes()
    });
    let child2 = thread::spawn(|| {
        compute_primes()
    });

    println!("{}", child1.join().unwrap());
    println!("{}", child2.join().unwrap());
}
