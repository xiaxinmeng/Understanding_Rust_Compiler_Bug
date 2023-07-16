rust
macro_rules! h {
    () => {
        static x: usize = 2;
        fn a(x: usize) -> usize {
            x + 2
        }
    }
}

h!();

fn main() {}
