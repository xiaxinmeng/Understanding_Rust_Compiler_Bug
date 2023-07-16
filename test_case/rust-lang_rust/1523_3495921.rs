
impl finally for fn@() {                                                                                                                           
    fn finally(f: block()) {                                                                                                                       
        resource r(f: fn@()) {                                                                                                                     
            f();                                                                                                                                   
        }                                                                                                                                          
        unsafe {                                                                                                                                   
            let _r = r(unsafe::reinterpret_cast(f));                                                                                               
            self();                                                                                                                                
        }                                                                                                                                          
    }                                                                                                                                              
}                                                                                                                                                  

fn main() {                                                                                                                                        
    let a = @"toast";                                                                                                                              
    fn@() {                                                                                                                                        
        log(error, "test");                                                                                                                        
    }.finally {||                                                                                                                                  
        log(error, a);                                                                                                                             
    }                                                                                                                                              
}  
