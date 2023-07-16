Rust
// src/lib.rs
// This file as-is results in an ICE

use simba;  //Comment this line results in "error: cannot determine resolution for the import"

#[cfg(test)]
mod tests {
    use approx; // Comment this line and the test completes successfully.

    #[test]
    fn simple_test(){
        use alga; // Comment this line and the test completes successfully.
        println!("Goodbye, world!");
    }
}

