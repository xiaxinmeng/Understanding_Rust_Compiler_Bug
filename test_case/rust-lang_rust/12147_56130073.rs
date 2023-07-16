 rust
/// Try to fetch a resource.  If the resource has not been loaded yet, block
/// until it is loaded.                                                     
fn fetch_block<'a>(&'a mut self, path: &str) -> Result<&'a Vec<u8>, E> {    
    loop {                                                                  
        match self.fetch(path) {                                            
            Ok(Some(x)) => { return Ok(x); },                               
            Ok(None) => { continue; },                                      
            Err(e) => { return Err(e); }                                    
        }                                                                   
    }                                                                       
}                          
