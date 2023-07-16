rust
    macro_rules! doc {
        (# [ doc = $s:tt ]) => { $s }
    }

    fn main() {
        // prints " informative "
        println!("{:?}", doc!(/** informative */));
    }
    