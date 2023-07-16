
// printlns are to prevent optimisation
#![feature(placement_in_syntax)]
#![feature(collection_placement)]
use std::collections::LinkedList;
fn main() {
    // [T; 10*1024*1024]
    let mut ll = LinkedList::new(); // EXAMPLE 1: T = u8
    ll.back_place() <- [0u8; 10*1024*1024];
    println!("{}", ll.front().unwrap()[0]+1);
    let mut ll = LinkedList::new(); // EXAMPLE 2: T = usize
    ll.back_place() <- [0usize; 10*1024*1024];
    println!("{}", ll.front().unwrap()[0]+1);

    // [[[[[T; 10]; 32]; 32]; 32]; 32] // 10*32^4 == 10*1024*1024
    let mut ll = LinkedList::new(); // EXAMPLE 3: T = u8
    ll.back_place() <- [[[[[0u8; 10]; 32]; 32]; 32]; 32];
    println!("{}", ll.front().unwrap()[0][0][0][0][0]+1);
    let mut ll = LinkedList::new(); // EXAMPLE 4: T = usize
    ll.back_place() <- [[[[[0usize; 10]; 32]; 32]; 32]; 32];
    println!("{}", ll.front().unwrap()[0][0][0][0][0]+1);
}
