rust
trait MyTrait {
    type Item;
}

pub trait Graph {
  type EdgeType;

  fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType>>;
}

impl<T> Graph for T {
  type EdgeType = T;

  fn adjacent_edges(&self) -> Box<dyn MyTrait<Item = &Self::EdgeType> + '_> {}
}
