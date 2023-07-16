 rust
#![feature(generic_const_exprs)]
use std::mem::align_of;

fn main() {
    println!("{}", align_of::<K>());  // 8
    println!("{}", align_of::<X<K>>());  // 8
}

struct K(u64);

struct X<A: Sized>
where
    (): AlignT<{ align_of::<A>() }>,
{
    k: [<() as AlignT<{ align_of::<A>() }>>::T; 0],
    x: u8,
}

trait AlignT<const N: usize> {
    type T: Copy;
}

impl AlignT<8> for () {
    type T = u64;
}

impl Copy for X<K> {}
impl Clone for X<K> {
    fn clone(&self) -> Self {
        *self
    }
}

