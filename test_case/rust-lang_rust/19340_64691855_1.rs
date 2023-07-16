
enumbin/src/main.rs:

extern crate enumice;        
use enumice::Foo;            

fn main() {
    let foo = Foo::Bar { name: format!("test") };
    match foo { 
        Foo::Bar { name } => {},        
    };
} 
