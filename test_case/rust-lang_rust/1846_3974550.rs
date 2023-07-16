
type foo<T> = {                                                                                                                                                               
        mutable f: option<@T>                                                                                                                                                 
};                                                                                                                                                                            

fn upd<T>(&&f: foo<T>, g: fn(T)) {                                                                                                                                            
    alt f.f {                                                                                                                                                                 
      some(v) {                                                                                                                                                               
            g(*(copy v));                                                                                                                                                     
            let x = v;                                                                                                                                                        
      }                                                                                                                                                                       
      none { }                                                                                                                                                                
    }                                                                                                                                                                         
}                                                                                                                                                                             

fn main() {                                                                                                                                                                   
    let x = @{mutable f: some(@3u)};                                                                                                                                          
    let y = x;                                                                                                                                                                
    upd(*x) {|v|                                                                                                                                                              
            y.f = none;                                                                                                                                                       
    }                                                                                                                                                                         
} 
