rust
#![feature(generic_associated_types)]

trait PointerFamily {
    type Pointer<T>;
}

struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = Box<T>;
}

enum Node<T> where <RcFamily as PointerFamily>::Pointer<Node<T>>: Sized {
    //Cons(<RcFamily as PointerFamily>::Pointer<Node<T>>, T),
    Cons(T, <RcFamily as PointerFamily>::Pointer<Node<T>>),
}

fn main() {
    let _list: Box<Node<i32>>;
}
