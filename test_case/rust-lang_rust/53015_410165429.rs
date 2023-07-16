rust
use std::time::SystemTime;

fn bench(input: &str) {
    let begin = SystemTime::now();
    for _ in 0..2000 {
        input.parse::<f64>().unwrap();
    }
    let elapsed = begin.elapsed().unwrap() / 2000;
    println!("{}: {:?}", input, elapsed);
}

fn main() {
    bench("9.9e305"); // ~300000ns
    bench("0.99e305"); // ~300ns
    bench("9.9e304"); // ~300ns
    
    bench("1.0e-288"); // ~200000ns
    bench("10.0e-288"); // ~306ns
    bench("1.0e-287"); // ~300ns
}
