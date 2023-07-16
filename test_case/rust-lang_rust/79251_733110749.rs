
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

struct Node<const D: usize>
where
    SmallVec<{ D * 2 }>: ,
{
    keys: SmallVec<{ D * 2 }>,
}

impl<const D: usize> Node<D>
where
    SmallVec<{ D * 2 }>: ,
{
    fn new() -> Self {
        let mut node = Node::new();
        node.keys.some_function();
        node
    }
}

struct SmallVec<const D: usize> {}
