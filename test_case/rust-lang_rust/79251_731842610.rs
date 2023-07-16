rust
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

#[derive(Debug)]
struct Node<K, const D: usize>
where
    SmallVec<K, { D * 2 }>: ,
{
    keys: SmallVec<K, { D * 2 }>,
}

impl<K, const D: usize> Node<K, D>
where
    SmallVec<K, { D * 2 }>: ,
{
    fn new() -> Self {
        panic!()
    }

    #[inline(never)]
    fn split(&mut self, i: usize, k: K, right: bool) -> Node<K, D> {
        let mut node = Node::new();
        node.keys.push(k);
        node
    }
}

#[derive(Debug)]
struct SmallVec<T, const D: usize> {
    data: [T; D],
}
impl<T, const D: usize> SmallVec<T, D> {
    fn new() -> Self {
        panic!()
    }
}
