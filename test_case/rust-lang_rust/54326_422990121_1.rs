rust
#![feature(generators, generator_trait)]

use std::ops::Generator;

fn foo() -> impl Generator<Return = i32> {
    || {
        if false {
            return Ok(6);
        }
        
        yield ();
    
        5
    }
}
