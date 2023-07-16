 rust
#[crate_type = "lib"];

mod inner {                      
    pub trait Trait {            
        fn f(&self) { f(); }     
    }                            

    impl Trait for int {}        

    fn f() {}                    
}                                

pub fn foo() {                   
    let a = &1 as &inner::Trait; 
    a.f();                       
}                                
