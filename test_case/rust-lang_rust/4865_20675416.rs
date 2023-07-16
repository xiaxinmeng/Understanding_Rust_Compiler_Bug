 rust
pub use hello::*; // doesn't work - `pub use hello::hello;` does

pub mod say {
    pub fn hello() { println("hello"); }
}

pub mod hello {
    use say;

    pub fn hello() {
        say::hello();
    }
}

fn main() {
    hello();
}

