
#![feature(box_syntax)]
use std::any::Any;
fn main()
{
    fn h(x:i32) -> i32 {3*x};
    fn g(x:i32) -> i32 {5*x + 3};

    // That works with explicit typing
    let mut vf:Vec<Box<Fn(i32)->i32>>=vec![];
    vf.push(box h);
    vf.push(box g);
    println!("3X={:?} 5X+3={:?}", vf[0](9), vf[1](3));

    // The following fails at the second push : type mismatch looks strange
    let mut vfnfer:Vec<Box<Any>> = vec![];
    vfnfer.push(box h);
    vfnfer.push(box g);
    println!("3X={:?} 5X+3={:?}", vf[0](9), vf[1](3));
}
