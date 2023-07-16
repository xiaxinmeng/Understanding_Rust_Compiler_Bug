
use std::ops::{Shr, Not};

trait MyTrait: 
  Shr<u32, Output=Self> + Shr<u64, Output=Self>
+ Not<Output=Self>
+ Sized {}
impl MyTrait for usize {}

fn neg_shift_right_by_one<T: MyTrait>(number: T) -> T {
    !(number >> 1)
}
