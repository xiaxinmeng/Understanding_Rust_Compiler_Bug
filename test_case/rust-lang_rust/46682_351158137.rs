rust
#![feature(target_feature)]    
#![crate_type = "lib"]         
                               
extern crate stdsimd;          
                               
use stdsimd::simd::*;          
use stdsimd::vendor::*;        
                               
#[target_feature = "+avx"]     
pub fn foo(a: u8x32) -> u8x32 {
    a.clone()                  
}                              
