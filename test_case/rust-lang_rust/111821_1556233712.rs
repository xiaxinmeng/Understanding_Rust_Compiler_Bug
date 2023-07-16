rust
#![feature(ptr_metadata)]

use core::ptr::Pointee;

struct Node<T> {
    element: T,
    next: <Node<T> as Pointee>::Metadata,
    prev: <Node<T> as Pointee>::Metadata,
    _tail: (),
}
