rust
#[derive(Debug, Clone)]
struct Wrapper(f64);

fn main() {
    println!("{:.7?}", vec![Wrapper(1.0); 4]);
}
