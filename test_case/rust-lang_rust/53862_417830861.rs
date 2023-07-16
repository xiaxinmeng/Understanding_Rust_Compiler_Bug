Rust
mod a {
    pub fn same_name() { println!("mod a") }
}

mod b {
    pub fn same_name() { println!("mod b") }
}

fn main() {
    {
        use a::*;
        same_name();
    }
    {
        use b::*;
        same_name();
    }
}
