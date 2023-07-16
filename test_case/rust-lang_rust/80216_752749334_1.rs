rust
iter::once(self.do_something(xyz))
    .chain(vec![1, 2, 3].into_iter())
    .chain(iter::once(self.my_function()))
