rust
use smallvec::SmallVec as SVec;

fn main() {
    let mut v: SVec<[u32; 2]> = smallvec::smallvec![1, 2, 3, 4, 5, 6, 7];
    println!("{}", v.capacity());
    v.push(1);
    println!("{}", v.capacity());
    v.push(1);
    println!("{}", v.capacity());
}
/* 
Prints
7
8
16
*/
