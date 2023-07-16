rust
std::iter::once(self.do_something(xyz))
    .chain(vec![1, 2, 3].into_iter())
    .chain(std::iter::once(self.my_function()))
