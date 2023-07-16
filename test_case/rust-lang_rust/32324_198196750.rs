 rust
extern crate gfx_core;                                              

#[derive(Copy)]                                                     
enum Command {                                                      
    BindConstantBuffers(gfx_core::pso::ConstantBufferSet<Resources>)
}                                                                   

enum Resources { }                                                  

impl gfx_core::Resources for Resources {}                           
