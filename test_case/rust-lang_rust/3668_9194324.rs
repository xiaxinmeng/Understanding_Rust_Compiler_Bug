 rust
struct P { child: Option<@mut P> }                                                                                                                                                                                                                                                                                                                                    
trait PTrait {                                                                                                                                                                     
   fn getChildOption() -> Option<@P>;                                                                                                                                              
}                                                                                                                                                                                  

impl P: PTrait {                                                                                                                                                                   
   fn getChildOption() -> Option<@P> {                                                                                                                                             
      let opt: Option<@P> = None;                                                                                                                                                        if self.child.is_some() {                                                                                                                                                             const childVal: @P = self.child.get();                                                                                                                                    
         opt = Some(childVal);                                                                                                                                                     
      }                                                                                                                                                                            
      opt                                                                                                                                                                          
   }                                                                                                                                                                               
}
