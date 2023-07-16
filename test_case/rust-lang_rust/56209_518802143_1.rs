rust

#[derive(Copy,Clone,Default,PartialEq,Eq,Hash)]
pub struct NodeIndex {
    data: usize,
}
#[derive(Copy,Clone,Default,PartialEq,Eq,Hash)]
pub struct EdgeIndex {
    data: usize,
}
impl<E,N> Index<NodeIndex> for Graph<E,N> {
    type Output = N;
    fn index<'b>(&'b self, indexer: NodeIndex) -> &'b N {
        self.nodes.index(indexer.data)
    }
}
impl<E,N> Index<EdgeIndex> for Graph<E,N> {
    type Output = E;
    fn index<'b>(&'b self, indexer: EdgeIndex) -> &'b E {
        self.nodes.index(indexer.data)
    }
}
