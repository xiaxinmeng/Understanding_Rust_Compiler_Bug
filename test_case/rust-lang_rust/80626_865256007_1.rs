
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

trait Allocator {
    type Allocated<T>;
}

enum LinkedList<'a, A: Allocator> {
    Head,
    Next(&'a A::Allocated<Self>)
}

fn main() {}
