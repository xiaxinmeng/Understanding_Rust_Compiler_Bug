rs
pub trait ConstGenericTrait<const N: u32> {}

impl ConstGenericTrait<{my_fn(2)}> for () {}

impl ConstGenericTrait<{my_fn(3)}> for () {}

impl ConstGenericTrait<{my_fn(1)}> for () {}

const fn my_fn(v: u32) -> u32 {
    if v == 2 { 
        panic!("Some error occurred")
    } else {
        v
    }
}
