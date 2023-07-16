rust
mod a {

    mod b {
        pub fn something() { } 
    }

    pub use b::*;
    use b::something; // overrides "pub use b::something"
    
    fn other() { something() }
}

fn main() {
    // error: "function something is private"!
    a::something();
}
