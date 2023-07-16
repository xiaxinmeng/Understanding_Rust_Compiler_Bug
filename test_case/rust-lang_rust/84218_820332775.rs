rust
use std::ops::Shr;

fn works() { 1_u64 >> 2_u32; }

fn breaks<T>()
where 
    u64: Shr<T>
{ 
    1_u64 >> 2_u32; 
}
