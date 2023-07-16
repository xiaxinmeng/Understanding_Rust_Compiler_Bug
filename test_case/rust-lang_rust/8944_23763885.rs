
fn select(&mut self) -> ~Method<'self> {                                   
    // Consume arms one at a time                                          
    loop {                                                                 
        let selector = ...;
        let pieces = self.collect();                                       
        arms.push(SelectArm { selector: selector, result: pieces });   
        ...
    }                                                                      
}                                                                          
