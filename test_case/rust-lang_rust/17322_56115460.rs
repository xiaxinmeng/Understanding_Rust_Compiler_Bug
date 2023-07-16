 rust
trait Foo {}               

fn foo(_: &[&Foo]) {}      

impl<'a> Foo for &'a str {}

fn main() {                
    foo(&[&"foo"]);        
}                          
