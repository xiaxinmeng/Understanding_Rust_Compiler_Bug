 rust
fn main() {
    let mut state = 54321;
    let a = || { 
        || { state += 1 }
    };

    a()();
}
