
unsafe impl<T: Send + ?Sized> Send for Unique<T> { }

pub struct Unique<T:?Sized> {
    pointer: *const T,
}

pub struct Node<V> {
    vals: V,
    edges: Unique<Node<V>>,
}

fn is_send<T: Send>() {}

fn main() {
    is_send::<Node<&'static ()>>();
}
