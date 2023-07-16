 rust
extern mod extra; 
mod foo { 
    extern mod extra; 
    pub fn bar() { 
        let x: ::extra::arc::Arc<uint> = ::foo::extra::arc::Arc::new(5u); 
        println!("{:?}", x); 
    }
} 
fn main() {
    foo::bar(); 
}
