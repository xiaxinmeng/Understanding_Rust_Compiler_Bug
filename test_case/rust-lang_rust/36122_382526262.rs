rust
fn main() {
    extern "C" {
        static symbol: [usize];
    }

    unsafe {
        println!("{}", symbol[0]);
    }
}
