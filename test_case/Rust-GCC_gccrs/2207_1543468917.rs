rust
macro_rules! different_repetitions {
    ($ a : expr, $ ($ b : expr), *) => { $ ($ a + $ b ;) * ; }
}

fn main() { 1 + 2; 1 + 3; 1 + 4; ; }
