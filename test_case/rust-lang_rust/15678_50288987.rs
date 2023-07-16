 rust
#![feature(overloaded_calls)]
use std::ops::{Fn, Add};
struct g;
impl<'a,A:Add<int,int>> Fn<(A,),int> for g {
    #[rust_call_abi_hack]
    fn call(&self, (arg,): (A,)) -> int {
        arg.add(&1)
    }
}
g(1i)
