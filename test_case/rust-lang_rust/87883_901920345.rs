rust
use smallvec::SmallVec as SVec;

fn main() {
    let mut v: Vec<u32> = vec!(1, 2, 3);
    println!("{}", v.capacity());
    for i in 0..71 {
        v.push(i);
    }
    println!("{}", v.capacity());

    let mut v: SVec<[u32; 2]> = smallvec::smallvec![1, 2, 3];
    println!("{}", v.capacity());
    for i in 0..71 {
        v.push(i);
    }
    println!("{}", v.capacity());
}
/*
Prints
3
96
3
128
*/
