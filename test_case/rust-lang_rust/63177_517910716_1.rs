rust
self.try_find(move |x| Ok::<_, !>(f(x))).unwrap()
