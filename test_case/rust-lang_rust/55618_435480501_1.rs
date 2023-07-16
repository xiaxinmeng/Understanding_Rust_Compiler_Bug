rust
#![feature(uniform_paths)]

mod bar {
    pub fn x() { }
}

fn main() {
    mod bar { pub fn y() { } }
    
    use bar::x;
    
    x();
}
