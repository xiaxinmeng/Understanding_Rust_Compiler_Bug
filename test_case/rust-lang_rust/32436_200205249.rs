 rust
#![feature(lang_items)]                                
#![feature(alloc)]                                     
#![no_std]                                             

extern crate alloc;                                    

use alloc::boxed::Box;                                 

pub fn foo(s: Box<i32>) {                              
    bar();                                             
}                                                      

pub fn bar() {}                                        

#[lang = "eh_unwind_resume"]                           
pub unsafe extern fn eh_unwind_resume(_args: *mut u8) {
}                                                      
