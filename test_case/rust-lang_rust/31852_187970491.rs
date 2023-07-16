 rust
trait Foo {                          
    fn foo<T>(&self, t: T);          
}                                    

impl Foo for i32 {                   
    fn foo<T: 'static>(&self, t: T) {
    }                                
}                                    

fn foo(a: i32, not_static: &i32) {   
    a.foo(not_static);               
}                                    
