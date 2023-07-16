 rust
#![feature(box_syntax)]
#![feature(core)]
#![feature(unboxed_closures)]

use std::any::Any;
use std::ops::Fn;

// desugared version of `|x| 3 * x`
struct Closure;

impl Fn<(i32,)> for Closure {
    type Output = i32;

    extern "rust-call" fn call(&self, (x,): (i32,)) -> i32 {
        3 * x
    }
}

fn main()
{
    // If the following works...
    {
        let bf: Box<Fn(_)->_> = box Closure;
        let bbf: &Any = &bf;
        let bf = bbf.downcast_ref::<Box<Fn(i32)->i32>>().unwrap();
        println!("Function evaluation gives {:?}",bf(6));
    }

    // then why does the following fail?
    {
        let bf: Box<Closure> = box Closure;
        let bbf:&Any = &bf;
        let bf = bbf.downcast_ref::<Box<Fn(i32)->i32>>().unwrap();
        println!("Function evaluation gives {:?}",bf(6));
    }
}
