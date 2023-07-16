rust
#![feature(const_trait_impl, const_option)]
use std::convert::TryFrom;

struct A(usize);
struct B(i32);

impl const TryFrom<A> for B {
    type Error = &'static str;
    fn try_from(x: A) -> Result<Self, &'static str> {
        let converted = x.0 as i32;
        if x.0 == converted as usize {
            Ok(B(converted))
        } else {
            Err("no")
        }
    }
}

const N: usize = 20;
const NI: i32 = B::try_from(A(N)).unwrap().0;
