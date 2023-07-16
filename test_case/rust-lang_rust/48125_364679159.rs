rust
#![crate_type = "cdylib"]          
                       
#[no_mangle]            
pub extern fn foo(a: u32) -> u32 { 
    a + 1                          
}                                  
