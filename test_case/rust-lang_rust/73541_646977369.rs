rust
fn main() {
    'a: loop {
        || {
            loop { continue 'a }
        };
        
    }
}
