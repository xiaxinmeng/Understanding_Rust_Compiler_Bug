 rust
pub fn each_loop(mut thing: &mut List, f: |&mut int|) {
    loop {                                             
        f(&mut thing.x);                               

        let tmp = thing;                               
        match tmp.next {                               
            None => break,                             
            Some(~ref mut next) => {                   
                thing = next;                          
            }                                          
        }                                              
    }                                                  
}                                                      
