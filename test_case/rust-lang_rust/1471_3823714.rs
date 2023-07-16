
type actor<T> = {                                                                                                                                                             
    unused: bool                                                                                                                                                              
};                                                                                                                                                                            

fn act2<T:send>() -> actor<T> {                                                                                                                                               
}                                                                                                                                                                             

enum in {                                                                                                                                                                     
    preprocess([u8]),                                                                                                                                                         
    exit                                                                                                                                                                      
}                                                                                                                                                                             

fn mk() -> actor<in> {                                                                                                                                                        
    act2()                                                                                                                                                                    
}                                                                                                                                                                             

fn main() {                                                                                                                                                                   
}                                                                                                                                                                             
