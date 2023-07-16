rust
    // src/main.rs

    #![feature(track_caller)]

    #[path = "module.rs"]
    mod a;

    #[path = "../src/module.rs"]
    mod b;

    fn main() {
        println!("{}", a::get().file());
        println!("{}", b::get().file());
    }
    