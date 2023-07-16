 rust
use std::os;                                           
use std::io::process;                                  

fn main () {                                           
    let args = os::args();                             
    if args.len() > 1 && args[1] == ~"child" {         
        for _ in range(0, 1000) {                      
            error!("hello?");                          
        }                                              
        for _ in range(0, 1000) {                      
            println!("hello?");                        
        }                                              
    }                                                  

    let args = [~"child"];                            
    let config = process::ProcessConfig {              
        program : args[0].as_slice(),                  
        args : args,                             
        env : None,                                    
        cwd : None,                                    
        io : []                                        
    };                                                 

    let mut p = process::Process::new(config).unwrap();
    println!("{}", p.wait());                          
}                                                      
