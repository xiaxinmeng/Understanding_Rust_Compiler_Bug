Rust
#![crate_type="rlib"]
#![feature(rustc_private)]
extern crate rustc;

use rustc::mir::*;

pub fn biggie2(basic_blocks: &mut Vec<BasicBlockData<'static>>,
               mk: fn() -> BasicBlockData<'static>,
               may_panic: fn())
{
    {
        let value = mk();
        may_panic();
        basic_blocks.push(value);
    }

    {
        let value = mk();
        may_panic();
        basic_blocks.push(value);
    }

    {
        let value = mk();
        may_panic();
        basic_blocks.push(value);
    }
}

