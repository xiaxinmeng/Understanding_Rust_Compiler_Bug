 rust
pub mod foo {         
    pub enum Bar {    
        Baz           
    }                 

    impl Bar {        
        fn where() { }
    }                 
}                     

fn main() {           
    use foo::Bar;     
    Bar::where(); // error here
}                     
