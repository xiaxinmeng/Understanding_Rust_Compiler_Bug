
match self.contents {
  Leaf(mut vs) => {
    if time_to_bisect {
      let (low, high) = Octree::bisect(vs);
      self.contents = Branch(Branches { low_tree: box low, high_tree: box high });
    } else {
      ...
    }
  },
  Branch(mut b) => {
      ...
  },
}
