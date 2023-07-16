rust
self.try_fold(init, move |acc, x| Ok::<B, !>(f(acc, x))).unwrap()
