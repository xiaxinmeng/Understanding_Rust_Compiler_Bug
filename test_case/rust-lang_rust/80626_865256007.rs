
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

trait Allocator {
    type Allocated<T>;
}

enum LinkedList<A: Allocator> {
    Head,
    Next(Box<A::Allocated<Self>>)
}

fn main() {}
