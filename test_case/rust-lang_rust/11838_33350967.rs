 rust
struct Foo<'a> {                   
    v: &'a int                     
}                                  

fn foo<'a>(f: &'a int) -> Foo<'a> {
    Foo { v: f }                   
}                                  

fn main() {                        
    let args = foo(&1);            
}                                  
