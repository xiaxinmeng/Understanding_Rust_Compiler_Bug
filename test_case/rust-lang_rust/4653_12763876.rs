 rust
struct A {                                    
  val: ~str                                   
}                                             

fn main() {                                   
  macro_rules! check_set (                    
    ($set:expr, $id:expr) => {                
      match &$id {                            
        _ => io::println(fmt!("%s", $id.val)) 
      }                                       
    }                                         
  );                                          
  let a = @A{ val: ~"" };                     
  check_set!(set, a);                         
}                                             
